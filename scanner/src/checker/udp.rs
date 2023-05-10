use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::UdpSocket;

pub async fn udp_success(socket: SocketAddr, timeout: u64) -> bool {
    let result = udp_send(socket, timeout).await;
    println!("{result:?}");
    result.is_ok()
}

async fn udp_send(socket: SocketAddr, timeout: u64) -> anyhow::Result<()> {
    let addr = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>()?).await?;

    addr.connect(socket).await?;
    addr.send(b"").await?;
    let mut buf = [0u8; 32];
    let recv = addr.recv(&mut buf).await;
    println!("recv: {recv:?}");
    // let a = tokio::time::timeout(
    //     Duration::from_secs(timeout),
    //     addr.connect(socket)
    // ).await??;
    Ok(())
}
