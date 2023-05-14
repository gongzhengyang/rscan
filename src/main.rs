use clap::Parser;
use std::time::Duration;

use scanner::execute;
use scanner::opts::{Executes, ScanOpts};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let scan_opts = ScanOpts::parse();
    scanner::performance::set_ulimit(1048567, 1048567).unwrap();
    let timeout = scan_opts.timeout;
    tracing::info!("waiting for {} seconds", timeout);
    match scan_opts.execute {
        Executes::Ping => {
            tracing::info!("execute icmp");
            execute::icmp::ping_ips(scan_opts.clone()).await.unwrap();
        }
        _ => {
            panic!("invalid protocol")
        }
    }

    tokio::time::sleep(Duration::from_secs(timeout)).await;
}
