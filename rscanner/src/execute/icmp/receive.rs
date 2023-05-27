use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use hashbrown::HashSet;
use pnet::packet::icmp::{echo_reply::EchoReplyPacket, IcmpTypes};
use pnet::packet::Packet;
use tokio::sync::OnceCell;

use super::common;

static RECEIVE_PACKETS: OnceCell<Arc<Mutex<HashSet<IpAddr>>>> = OnceCell::const_new();

pub async fn receive_packets_handle() -> &'static Arc<Mutex<HashSet<IpAddr>>> {
    RECEIVE_PACKETS
        .get_or_init(|| async { Arc::new(Mutex::new(HashSet::new())) })
        .await
}

pub async fn is_addr_received(addr: &IpAddr) -> bool {
    receive_packets_handle()
        .await
        .lock()
        .unwrap()
        .contains(addr)
}

pub async fn receive_packets() -> anyhow::Result<()> {
    let (_, mut rx) = common::get_transport_channel()?;
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        tokio::time::sleep(Duration::from_nanos(1)).await;
        if let Ok(Some((packet, addr))) = iter.next_with_timeout(Duration::from_secs(1)) {
            if is_addr_received(&addr).await {
                return Ok(());
            }
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                if reply_packet.get_icmp_type() == IcmpTypes::EchoReply {
                    println!("rscan|icmp|{addr}|");
                    let mut receive_handle = receive_packets_handle().await.lock().unwrap();
                    receive_handle.insert(addr);
                }
            }
        }
    }
}
