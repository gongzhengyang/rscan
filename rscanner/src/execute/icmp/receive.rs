use std::time::Duration;

use pnet::packet::icmp::{echo_reply::EchoReplyPacket, IcmpTypes};
use pnet::packet::Packet;

use super::common;
use crate::monitor;

pub async fn receive_packets() -> anyhow::Result<()> {
    let (_, mut rx) = common::get_transport_channel()?;
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        tokio::time::sleep(Duration::from_micros(10)).await;
        if let Ok(Some((packet, addr))) = iter.next_with_timeout(Duration::from_millis(1)) {
            if monitor::is_addr_received(&addr).await {
                continue;
            }
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                if reply_packet.get_icmp_type() == IcmpTypes::EchoReply {
                    println!("rscan|icmp|{addr}|");
                    monitor::add_receive_ipaddr(addr).await;
                }
            }
        }
    }
}
