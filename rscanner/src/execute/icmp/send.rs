use std::net::{IpAddr, Ipv4Addr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use tokio::time::MissedTickBehavior;

use crate::monitor;
use crate::setting::command::ScanOpts;

use super::common;
use super::interface;
use super::receive;

static SEND: AtomicU64 = AtomicU64::new(0);

pub async fn scan(scan_opts: ScanOpts) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(scan_opts.retry_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let timeout = scan_opts.timeout;
    tracing::info!("rscanner timeout is {timeout}");
    tokio::spawn(async move {
        for _ in 0..scan_opts.retry + 1 {
            for chunk_hosts in scan_opts.hosts.chunks(scan_opts.batch_size) {
                let chunk_hosts_cloned = chunk_hosts.to_vec();
                tokio::spawn(async move {
                    icmp_ips_chunks(chunk_hosts_cloned).await.unwrap();
                });
            }
            interval.tick().await;
        }
    });
    match tokio::time::timeout(Duration::from_secs(timeout), receive::receive_packets()).await {
        Err(_) => {
            tracing::info!("receive packets thread over because timeout");
            let send_count = SEND.load(Ordering::Relaxed);
            let total_received = monitor::receive_packets_handle().await.lock().await.len();
            println!("send {send_count} ips, receive packets from {total_received} ips");
        }
        Ok(e) => {
            tracing::error!("something wrong with {e:?}");
        }
    }
    Ok(())
}

pub async fn icmp_ips_chunks(hosts: Vec<Ipv4Addr>) -> anyhow::Result<()> {
    let (mut tx, _) = common::get_transport_channel()?;
    for host in hosts {
        let target = IpAddr::from(host);
        if monitor::is_addr_received(&target).await {
            tracing::debug!("skip target {target} because received");
            continue;
        }
        let mut header = [0u8; common::ICMP_LEN];
        let mut icmp_packet = MutableEchoRequestPacket::new(&mut header).unwrap();
        common::set_icmp_send_packet(&mut icmp_packet);
        tracing::debug!("build icmp success for {host}");
        if tx.send_to(icmp_packet, target).is_err() {
            interface::send_with_interface(host).unwrap_or_default();
        }
        SEND.fetch_add(1, Ordering::Relaxed);
    }
    Ok(())
}
