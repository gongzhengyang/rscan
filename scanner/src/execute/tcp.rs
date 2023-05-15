use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpStream;
use crate::opts::ScanOpts;

pub async fn tcp_success(socket: SocketAddr, timeout: u64) {
    if tcp_connect(socket, timeout).await.is_ok() {
        println!("rscan|tcp|{socket}|");
    }
}

pub async fn tcp_connect(socket: SocketAddr, timeout: u64) -> anyhow::Result<TcpStream> {
    Ok(tokio::time::timeout(Duration::from_secs(timeout), TcpStream::connect(socket)).await??)
}

pub async fn scan(scan_opts: ScanOpts) -> anyhow::Result<()> {
    for socket_addr in scan_opts.iter_sockets()? {
        let per_timeout = scan_opts.per_timeout;
        tokio::spawn(async move {
            tcp_success(socket_addr, per_timeout).await
        });
    }
    tokio::time::sleep(Duration::from_secs(scan_opts.timeout)).await;
    Ok(())
}
