use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // for port in 161..165 {
    //     tokio::spawn(async move {
    //         let socket = SocketAddr::from((IpAddr::V4(Ipv4Addr::from([10, 30, 6, 151])), port));
    //         if scanner::checker::udp_success(socket, 5).await {
    //             println!("{socket}");
    //         }
    //     });
    // }
    let target_ip = "10.30.6.151".parse::<IpAddr>().unwrap();
    scanner::checker::icmp::ping_ip(target_ip).unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Hello, world!");
}
