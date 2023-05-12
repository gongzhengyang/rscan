use std::net::{IpAddr, Ipv4Addr};
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

use crate::opts::ScanOpts;

const ICMP_LEN: usize = 64;

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

pub async fn ping_ips(scan_opts: ScanOpts) -> anyhow::Result<()> {
    let channel_type = pnet_transport::TransportChannelType::Layer4(
        pnet_transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp),
    );
    let hosts_len= scan_opts.hosts.len();
    let (mut tx, rx) = pnet_transport::transport_channel(ICMP_LEN * (hosts_len + 1), channel_type)?;
    let timeout = scan_opts.timeout;
    tokio::spawn(async move {
        tokio::time::timeout(Duration::from_secs(timeout), receive_packets(rx))
            .await
            .unwrap_or_else(|e| tracing::info!("icmp packet receiver stopped because timeout"));
    });
    let mut interval = tokio::time::interval(Duration::from_secs(scan_opts.retry_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let now = tokio::time::Instant::now();

    let mut count = 0;
    for retry in 0..scan_opts.retries + 1 {
        for target_ip in &*scan_opts.hosts {
            let mut header = [0u8; ICMP_LEN];
            let mut icmp_packet = MutableEchoRequestPacket::new(&mut header).unwrap();
            modify_icmp_packet(&mut icmp_packet);
            tracing::debug!("build icmp success for {target_ip}");

            count += 1;
            tx.send_to(icmp_packet, IpAddr::from(target_ip.clone())).unwrap();
            tracing::debug!("send packets {count}");
        }
        tracing::info!("round[{retry}] sending {hosts_len} packets cost {} millis", now.elapsed().as_millis());
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
