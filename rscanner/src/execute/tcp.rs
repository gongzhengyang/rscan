use std::net::SocketAddr;
use std::time::Duration;

use async_trait::async_trait;
use tokio::net::TcpStream;

use super::common::SocketScanner;

pub struct TcpSocketScanner;

#[async_trait]
impl SocketScanner for TcpSocketScanner {
    async fn socket_success(socket: SocketAddr, timeout: u64) {
        tracing::debug!("trying connect socket {socket} with timeout {timeout}");
        if Self::socket_connect(socket, timeout).await.is_ok() {
            println!("rscan|tcp|{socket}|");
        }
    }

    async fn socket_connect(socket: SocketAddr, timeout: u64) -> anyhow::Result<()> {
        tokio::time::timeout(Duration::from_secs(timeout), TcpStream::connect(socket)).await??;
        Ok(())
    }
}
