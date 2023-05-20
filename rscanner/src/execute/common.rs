use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use async_trait::async_trait;

use crate::err::APPError;
use crate::opts::ScanOpts;
use crate::sockets_iter::SocketIterator;

#[async_trait]
pub trait SocketScanner {
    async fn socket_success(socket: SocketAddr, timeout: u64);

    async fn socket_connect(socket: SocketAddr, timeout: u64) -> anyhow::Result<()>;

    async fn pre_scan(_scan_opts: &ScanOpts) -> anyhow::Result<()> {
        Ok(())
    }

    async fn scan(scan_opts: ScanOpts) -> anyhow::Result<()> {
        Self::pre_scan(&scan_opts).await?;
        let ips = scan_opts
            .hosts
            .iter()
            .map(|x| IpAddr::V4(*x))
            .collect::<Vec<IpAddr>>();
        let ports = scan_opts.ports.ok_or(APPError::PortIsEmpty)?;
        let socket_iter = SocketIterator::new(&ips, &ports);
        for socket_addr in socket_iter {
            let per_timeout = scan_opts.per_timeout;
            tokio::spawn(async move { Self::socket_success(socket_addr, per_timeout).await });
        }
        tokio::time::sleep(Duration::from_secs(scan_opts.timeout)).await;
        Self::after_scan().await?;
        Ok(())
    }

    async fn after_scan() -> anyhow::Result<()> {
        Ok(())
    }
}
