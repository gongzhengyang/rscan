use std::net::{IpAddr, Ipv4Addr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use tokio::time::MissedTickBehavior;

use crate::opts::ScanOpts;

use super::common;
use super::interface;
use super::receive;

static R: AtomicU64 = AtomicU64::new(0);

pub async fn scan(scan_opts: ScanOpts) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(scan_opts.retry_interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let timeout = scan_opts.timeout;
    tracing::info!("scanner timeout is {timeout}");
    tokio::spawn(async move {
        for _ in 0..scan_opts.retry + 1 {
            for chunk_hosts in scan_opts.hosts.chunks(scan_opts.batch_size) {
                let chunk_hosts_cloned = chunk_hosts.to_vec();
                tokio::spawn(async move {
                    ping_ips_chunks(chunk_hosts_cloned).await.unwrap();
                });
            }
            interval.tick().await;
        }
    });
    let result =
        tokio::time::timeout(Duration::from_secs(timeout), receive::receive_packets()).await;
    match result {
        Err(_) => {
            tracing::info!("receive packets thread over because timeout");
            let send_count = R.load(Ordering::Relaxed);
            let total_received = receive::receive_packets_handle()
                .await
                .lock()
                .unwrap()
                .len();
            println!("send {send_count} ips, receive packets from {total_received} ips");
        }
        Ok(e) => {
            tracing::error!("something wrong with {e:?}");
        }
    }
    Ok(())
}

pub async fn ping_ips_chunks(hosts: Vec<Ipv4Addr>) -> anyhow::Result<()> {
    let (mut tx, _) = common::get_transport_channel()?;
    for host in hosts {
        let mut header = [0u8; common::ICMP_LEN];
        let mut icmp_packet = MutableEchoRequestPacket::new(&mut header).unwrap();
        common::modify_icmp_packet(&mut icmp_packet);
        tracing::debug!("build icmp success for {host}");
        let target = IpAddr::from(host);
        if receive::is_addr_received(&target).await {
            continue;
        }
        tx.send_to(icmp_packet, target).unwrap_or_else(|_| {
            interface::send_with_interface(host);
            0
        });
        // tx.send_to(icmp_packet, target).unwrap();
        R.fetch_add(1, Ordering::Relaxed);
        tracing::debug!("sending packets count {}", R.load(Ordering::Relaxed));
    }

    Ok(())
}
