use std::net::IpAddr;
use std::time::Duration;

use pnet::packet::{
    icmp::{
        echo_reply::EchoReplyPacket,
        echo_request::{IcmpCodes, MutableEchoRequestPacket},
        IcmpTypes,
    },
    ip::IpNextHeaderProtocols,
    Packet,
};
use pnet_transport::TransportReceiver;
use tokio::time::MissedTickBehavior;

async fn receive_packets(mut rx: TransportReceiver) {
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        if let Ok((packet, addr)) = iter.next() {
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                if reply_packet.get_icmp_type() == IcmpTypes::EchoReply {
                    println!("icmp receive {addr}");
                }
            }
        }
    }
}

pub async fn ping_ips(
    target_ips: Vec<IpAddr>,
    retries: u8,
    retry_interval: u64,
) -> anyhow::Result<()> {
    let channel_type = pnet_transport::TransportChannelType::Layer4(
        pnet_transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp),
    );
    let (mut tx, rx) = pnet_transport::transport_channel(4096, channel_type).unwrap();
    tokio::spawn(async move {
        receive_packets(rx).await;
    });
    let mut interval = tokio::time::interval(Duration::from_secs(retry_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    for _ in 0..retries {
        for target_ip in &target_ips {
            let mut header = [0u8; 64];
            let mut icmp_packet = MutableEchoRequestPacket::new(&mut header).unwrap();
            modify_icmp_packet(&mut icmp_packet);
            tx.send_to(icmp_packet, target_ip.clone())?;
        }
        interval.tick().await;
    }
    Ok(())
}

fn modify_icmp_packet(icmp_packet: &mut MutableEchoRequestPacket) {
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(rand::random::<u16>());
    icmp_packet.set_sequence_number(1);

    let checksum = pnet::packet::util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);
}
