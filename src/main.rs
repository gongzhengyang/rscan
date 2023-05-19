use clap::Parser;

use scanner::execute;
use scanner::opts::{Executes, ScanOpts};

#[tokio::main]
async fn main() {
    console_subscriber::init();
    let scan_opts = ScanOpts::parse();
    scanner::performance::set_ulimit(1048567, 1048567).unwrap();
    let timeout = scan_opts.timeout;
    tracing::info!("waiting for {} seconds", timeout);
    match scan_opts.execute {
        Executes::Ping => {
            tracing::info!("execute icmp scan");
            execute::icmp::scan(scan_opts.clone()).await.unwrap();
        }
        Executes::Tcp => {
            tracing::info!("execute tcp scan");
            execute::tcp::scan(scan_opts).await.unwrap();
        }
        _ => {
            panic!("invalid protocol")
        }
    }
}
