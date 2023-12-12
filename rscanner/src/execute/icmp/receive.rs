use std::time::Duration;

use pnet::packet::{
    icmp::{echo_reply::EchoReplyPacket, IcmpTypes},
    Packet,
};

use crate::monitor;

use super::common;

pub async fn receive_packets() -> anyhow::Result<()> {
    let (_, mut rx) = common::get_transport_channel()?;
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        tokio::time::sleep(Duration::from_millis(1)).await;
        while let Ok(Some((packet, addr))) = iter.next_with_timeout(Duration::from_millis(1)) {
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                if reply_packet.get_icmp_type() == IcmpTypes::EchoReply {
                    tracing::debug!("receive {addr}");
                    if monitor::add_receive_ipaddr(addr).await {
                        println!("rscan|icmp|{addr}|");
                    }
                }
            }
        }
    }
}
