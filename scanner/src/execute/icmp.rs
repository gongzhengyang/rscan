use hashbrown::HashSet;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
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

use crate::monitor;
use crate::opts::ScanOpts;

const ICMP_LEN: usize = 64;
static R: AtomicU64 = AtomicU64::new(0);
static RECEIVED_PACKETS: HashSet<Ipv4Addr> = HashSet::new();

async fn receive_packets(mut rx: TransportReceiver) {
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        tokio::time::sleep(Duration::from_micros(1)).await;
        if let Ok((packet, addr)) = iter.next() {
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                if reply_packet.get_icmp_type() == IcmpTypes::EchoReply
                    && !RECEIVED_PACKETS.contains(&addr)
                {
                    println!("icmp receive {addr}");
                }
            }
        }
    }
}

fn modify_icmp_packet(icmp_packet: &mut MutableEchoRequestPacket) {
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(rand::random::<u16>());
    icmp_packet.set_sequence_number(1);

    let checksum = pnet::packet::util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);
}

pub async fn ping_ips(scan_opts: ScanOpts) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(scan_opts.retry_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let now = tokio::time::Instant::now();
    let timeout = scan_opts.timeout;
    for retry in 0..scan_opts.retries + 1 {
        let mut chunks = scan_opts.hosts.chunks(scan_opts.batch_size);
        while let Some(chunk_hosts) = chunks.next() {
            let chunk_hosts_cloned = chunk_hosts.clone().to_vec();
            tokio::spawn(async move {
                ping_ips_chunks(chunk_hosts_cloned, timeout).await.unwrap();
            });
        }
        tracing::info!(
            "round[{retry}] sending packets cost {} millis",
            now.elapsed().as_millis()
        );
        interval.tick().await;
    }
    tokio::time::sleep(Duration::from_secs(scan_opts.timeout)).await;
    monitor::display_send_ip_monitor().await;
    Ok(())
}

pub async fn ping_ips_chunks(hosts: Vec<Ipv4Addr>, timeout: u64) -> anyhow::Result<()> {
    let channel_type = pnet_transport::TransportChannelType::Layer4(
        pnet_transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp),
    );
    let (mut tx, mut rx) = pnet_transport::transport_channel(4096, channel_type)?;
    for host in hosts {
        let mut header = [0u8; ICMP_LEN];
        let mut icmp_packet = MutableEchoRequestPacket::new(&mut header).unwrap();
        modify_icmp_packet(&mut icmp_packet);
        tracing::debug!("build icmp success for {host}");
        tx.send_to(icmp_packet, IpAddr::from(host.clone())).unwrap();
        R.fetch_add(1, Ordering::Relaxed);
        monitor::add_send_ip_monitor(host).await;
        tracing::debug!("sending packets count {}", R.load(Ordering::Relaxed));
    }
    // tokio::spawn(async move {
    //     tokio::time::timeout(Duration::from_secs(timeout), receive_packets(rx))
    //         .await
    //         .unwrap_or_else(|e| tracing::info!("icmp packet receiver stopped because timeout"));
    // });
    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);
    loop {
        tokio::time::sleep(Duration::from_micros(1)).await;
        if let Ok((packet, addr)) = iter.next() {
            // println!("{:?}--{:?}", packet, packet.payload());
            if let Some(reply_packet) = EchoReplyPacket::new(packet.packet()) {
                if reply_packet.get_icmp_type() == IcmpTypes::EchoReply {
                    println!("icmp receive {addr}");
                }
            }
        }
    }
    Ok(())
}
