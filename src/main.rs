use std::time::Duration;

use clap::Parser;

use rscanner::execute;
use rscanner::execute::common::SocketScanner;
use rscanner::setting::command::{ScanOpts, ScanType};

#[tokio::main]
async fn main() {
    // used for tokio task manage
    // console_subscriber::init();
    tracing_subscriber::fmt::init();
    let scan_opts = ScanOpts::parse();
    #[cfg(unix)]
    rscanner::performance::improve_limits().unwrap();
    let timeout = scan_opts.timeout;
    tracing::info!("waiting for {} seconds", timeout);

    let result_save_path = scan_opts.filepath.clone().unwrap_or_else(|| {
        format!(
            "tmp/{}.txt",
            chrono::Local::now().format("%Y-%m-%d--%H:%M:%S")
        )
    });
    match scan_opts.execute {
        ScanType::Icmp => {
            tracing::info!("execute icmp scan");
            tokio::spawn(async move {
                execute::icmp::scan(scan_opts.clone()).await.unwrap();
            });
        }
        ScanType::Tcp => {
            tracing::info!("execute tcp scan");
            tokio::spawn(async move {
                execute::tcp::TcpSocketScanner::scan(scan_opts)
                    .await
                    .unwrap();
            });
        }
        ScanType::Udp => {
            tracing::info!("execute udp scan");
            tracing::warn!(
                "udp scan based on icmp reply with Port Unreachable with udp packets,\
             please make sure timeout is big enough to receive all icmp for all udp packets"
            );
            tokio::spawn(async move {
                execute::udp::UdpSocketScanner::scan(scan_opts)
                    .await
                    .unwrap();
            });
        }
        ScanType::Arp => {
            tracing::info!("execute arp scan");
            tokio::spawn(async move {
                execute::arp::scan(scan_opts).await.unwrap();
            });
        }
        ScanType::Show => {
            println!("hosts len {}", scan_opts.hosts.len());
            let socket_iter = scan_opts.iter_sockets().unwrap();
            for socket in socket_iter {
                let scan_opts_cloned = scan_opts.clone();
                tokio::spawn(async move {
                    assert_eq!(scan_opts_cloned.timeout, 30);
                    if socket.ip().is_multicast() && socket.port() == 9999 {
                        println!("{}", socket);
                    }
                    tokio::time::sleep(Duration::from_secs(10)).await;
                });
            }
        }
    }
    tokio::time::sleep(Duration::from_secs(timeout + 1)).await;
    rscanner::monitor::save_receive_addrs(&result_save_path)
        .await
        .unwrap();
}
