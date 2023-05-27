use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use async_trait::async_trait;

use crate::setting::command::ScanOpts;

#[async_trait]
pub trait SocketScanner {
    async fn socket_success(socket: SocketAddr, timeout: u64);

    async fn socket_connect(socket: SocketAddr, timeout: u64) -> anyhow::Result<()>;

    async fn pre_scan(_scan_opts: &ScanOpts) -> anyhow::Result<()> {
        Ok(())
    }

    async fn pre_send_socket(_socket: &SocketAddr) -> anyhow::Result<()> {
        Ok(())
    }

    async fn scan(scan_opts: ScanOpts) -> anyhow::Result<()> {
        Self::pre_scan(&scan_opts).await?;

        // for socket_addr in &scan_opts.iter_sockets()? {
        //     let per_timeout = scan_opts.per_timeout;
        //     Self::pre_send_socket(&socket_addr)
        //         .await
        //         .unwrap_or_else(|e| tracing::error!("pre send socket error with {e:?}"));
        //     tokio::spawn(async move { Self::socket_success(socket_addr, per_timeout).await });
        // }

        Self::after_scan().await?;
        Ok(())
    }

    async fn after_scan() -> anyhow::Result<()> {
        Ok(())
    }
}
