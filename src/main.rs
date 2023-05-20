use clap::Parser;

use rscanner::execute;
use rscanner::execute::common::SocketScanner;
use rscanner::opts::{Executes, ScanOpts};

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
            execute::icmp::scan(scan_opts.clone()).await.unwrap();
        }
        Executes::Tcp => {
            tracing::info!("execute tcp scan");
            execute::tcp::TcpSocketScanner::scan(scan_opts)
                .await
                .unwrap();
        }
        Executes::Udp => {
            tracing::info!("execute udp scan");
            tracing::warn!(
                "udp scan based on icmp reply with Port Unreachable with udp packets,\
             please make sure timeout is big enough to receive all icmp for all udp packets"
            );
            execute::udp::UdpSocketScanner::scan(scan_opts)
                .await
                .unwrap();
        }
    }
}
