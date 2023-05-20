use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use hashbrown::HashSet;
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use tokio::net::UdpSocket;
use tokio::sync::OnceCell;

use crate::opts::ScanOpts;

use super::common::SocketScanner;
use super::icmp;

pub struct UdpSocketScanner;

static UDP_SOCKET: OnceCell<UdpSocket> = OnceCell::const_new();
static SOCKET_MANAGER: OnceCell<Arc<Mutex<HashSet<SocketAddr>>>> = OnceCell::const_new();

async fn get_udp_socket() -> &'static UdpSocket {
    UDP_SOCKET
        .get_or_init(|| async {
            UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())
                .await
                .unwrap()
        })
        .await
}

async fn get_socket_manager() -> &'static Arc<Mutex<HashSet<SocketAddr>>> {
    SOCKET_MANAGER
        .get_or_init(|| async { Arc::new(Mutex::new(HashSet::new())) })
        .await
}

#[async_trait]
impl SocketScanner for UdpSocketScanner {
    async fn socket_success(socket: SocketAddr, timeout: u64) {
        let result = Self::socket_connect(socket, timeout).await;
        if result.is_ok() {
            println!("{result:?}");
        }
    }

    async fn socket_connect(socket: SocketAddr, _timeout: u64) -> anyhow::Result<()> {
        let udp_socket = get_udp_socket().await;
        udp_socket.connect(socket.clone()).await?;
        udp_socket.send(b"").await?;
        let mut socket_manager = get_socket_manager().await.lock().unwrap();
        socket_manager.insert(socket);
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
}

async fn receive_icmp_packets() -> anyhow::Result<()> {
    let (_, mut rx) = icmp::common::get_transport_channel()?;
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        tokio::time::sleep(Duration::from_nanos(1)).await;
        if let Ok(Some((packet, addr))) = iter.next_with_timeout(Duration::from_secs(1)) {
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                let ipv4_packet = Ipv4Packet::new(reply_packet.payload());
                println!("{ipv4_packet:?}");
                let udp_packet = UdpPacket::new(reply_packet.payload());
                println!("{udp_packet:?}");
                println!("{reply_packet:?}--{:?}", reply_packet.payload());
            }
        }
    }
}
