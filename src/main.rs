use std::time::Duration;

use clap::Parser;

use rscanner::execute;
use rscanner::execute::common::SocketScanner;
use rscanner::setting::command::{Executes, ScanOpts};

#[tokio::main]
async fn main() {
    // used for tokio task manage
    console_subscriber::init();
    let scan_opts = ScanOpts::parse();
    #[cfg(unix)]
    rscanner::performance::improve_limits().unwrap();
    let timeout = scan_opts.timeout;
    tracing::info!("waiting for {} seconds", timeout);

    match scan_opts.execute {
        Executes::Icmp => {
            tracing::info!("execute icmp scan");
            tokio::spawn(async move {
                execute::icmp::scan(scan_opts.clone()).await.unwrap();
            });
        }
        Executes::Tcp => {
            tracing::info!("execute tcp scan");
            tokio::spawn(async move {
                execute::tcp::TcpSocketScanner::scan(scan_opts)
                    .await
                    .unwrap();
            });

        }
        Executes::Udp => {
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
        Executes::Arp => {
            tracing::info!("execute arp scan");
            tokio::spawn(async move {
                execute::arp::scan(scan_opts).await.unwrap();
            });
        }
        Executes::Show => {
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
}
