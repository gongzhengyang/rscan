use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpStream;

pub async fn tcp_success(socket: SocketAddr, timeout: u64) -> bool {
    tcp_connect(socket, timeout).await.is_ok()
}

async fn tcp_connect(socket: SocketAddr, timeout: u64) -> anyhow::Result<TcpStream> {
    Ok(tokio::time::timeout(Duration::from_secs(timeout), TcpStream::connect(socket)).await??)
}
