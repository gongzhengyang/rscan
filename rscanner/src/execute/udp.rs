use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use hashbrown::{HashMap, HashSet};
use pnet::packet::icmp::destination_unreachable::IcmpCodes;
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use tokio::net::UdpSocket;
use tokio::sync::OnceCell;
use tokio::time::Instant;

use crate::setting::command::ScanOpts;

use super::common::SocketScanner;
use super::icmp;

pub struct UdpSocketScanner;

static UDP_LAST_IP_SEND: OnceCell<Arc<Mutex<HashMap<IpAddr, Instant>>>> = OnceCell::const_new();
static UDP_UNREACHABLE_ADDRS: OnceCell<Arc<Mutex<HashSet<IpAddr>>>> = OnceCell::const_new();
static SOCKET_MANAGER: OnceCell<Arc<Mutex<HashSet<SocketAddr>>>> = OnceCell::const_new();

async fn get_upd_last_ip_send() -> &'static Arc<Mutex<HashMap<IpAddr, Instant>>> {
    UDP_LAST_IP_SEND
        .get_or_init(|| async { Arc::new(Mutex::new(HashMap::new())) })
        .await
}

async fn get_udp_unreachable_addrs() -> &'static Arc<Mutex<HashSet<IpAddr>>> {
    UDP_UNREACHABLE_ADDRS
        .get_or_init(|| async { Arc::new(Mutex::new(HashSet::new())) })
        .await
}

async fn add_udp_unreachable_addrs(addr: IpAddr) {
    let mut handle = get_udp_unreachable_addrs().await.lock().unwrap();
    handle.insert(addr);
}

async fn ip_udp_send_interval_millis(ip: IpAddr) -> u128 {
    let udp_last_send = get_upd_last_ip_send().await.lock().unwrap();
    let last_send = udp_last_send.get(&ip);
    if let Some(last_send) = last_send {
        let elapsed = last_send.elapsed().as_millis();
        if elapsed < 500 {
            return 500 - elapsed;
        }
    }
    0
}

async fn get_socket_manager() -> &'static Arc<Mutex<HashSet<SocketAddr>>> {
    SOCKET_MANAGER
        .get_or_init(|| async { Arc::new(Mutex::new(HashSet::new())) })
        .await
}

async fn add_socket_to_manager(socket: SocketAddr) {
    let mut socket_manager = get_socket_manager().await.lock().unwrap();
    socket_manager.insert(socket);
}

async fn remove_socket_from_manager(socket: &SocketAddr) {
    let mut socket_manager = get_socket_manager().await.lock().unwrap();
    let remove = socket_manager.remove(socket);
    tracing::debug!("remove {socket} {remove}");
}

#[async_trait]
impl SocketScanner for UdpSocketScanner {
    async fn socket_success(socket: SocketAddr, timeout: u64) {
        Self::socket_connect(socket, timeout)
            .await
            .unwrap_or_else(|e| tracing::error!("sending packets error with {e:?}"));
    }

    async fn socket_connect(socket: SocketAddr, _timeout: u64) -> anyhow::Result<()> {
        let udp_socket = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())
            .await
            .unwrap();
        udp_socket.connect(socket).await?;
        udp_socket.send(b"t").await?;
        tracing::debug!("send socket {socket:?}");
        add_socket_to_manager(socket).await;
        Ok(())
    }

    async fn pre_scan(scan_opts: &ScanOpts) -> anyhow::Result<()> {
        let timeout = scan_opts.timeout;
        tokio::spawn(async move {
            let result =
                tokio::time::timeout(Duration::from_secs(timeout), receive_icmp_packets()).await;
            if let Ok(e) = result {
                tracing::error!("something wrong with {e:?}");
            }
        });
        Ok(())
    }

    async fn pre_send_socket(socket: &SocketAddr) -> anyhow::Result<()> {
        let ip = socket.ip();
        let sleep_millis = ip_udp_send_interval_millis(ip).await;
        tokio::time::sleep(Duration::from_millis(sleep_millis as u64)).await;
        let mut last_send = get_upd_last_ip_send().await.lock().unwrap();
        last_send.insert(ip, Instant::now());
        Ok(())
    }

    async fn after_scan() -> anyhow::Result<()> {
        let udp_unreachable_addrs = {
            let handle = get_udp_unreachable_addrs().await.lock().unwrap();
            (*handle).clone()
        };
        let socket_manager = get_socket_manager().await.lock().unwrap();
        for socket in &*socket_manager {
            if udp_unreachable_addrs.contains(&socket.ip()) {
                println!("rscan|udp|{socket}|");
            }
        }
        Ok(())
    }
}

async fn receive_icmp_packets() -> anyhow::Result<()> {
    let (_, mut rx) = icmp::common::get_transport_channel()?;
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        tokio::time::sleep(Duration::from_micros(10)).await;
        if let Ok(Some((packet, addr))) = iter.next_with_timeout(Duration::from_secs(1)) {
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                if reply_packet.get_icmp_code() == IcmpCodes::DestinationPortUnreachable {
                    if let Some(ipv4_packet) = Ipv4Packet::new(reply_packet.payload()) {
                        if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                            let socket = SocketAddr::new(
                                IpAddr::V4(ipv4_packet.get_destination()),
                                udp_packet.get_destination(),
                            );
                            tracing::debug!("unreachable socket is {socket:?}");
                            remove_socket_from_manager(&socket).await;
                            add_udp_unreachable_addrs(addr).await;
                        }
                    }
                }
            }
        }
    }
}
