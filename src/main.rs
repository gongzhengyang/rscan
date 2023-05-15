use clap::Parser;
use std::time::Duration;

use scanner::execute;
use scanner::opts::{Executes, ScanOpts};

#[tokio::main]
async fn main() {
    // let interfaces = pnet::datalink::interfaces();
    // for i in interfaces {
    //     i.is_up()
    // if i.is_up() && !i.is_loopback() {
    //     println!(
    //         "{:?} up[{}] running[{}] broad[{}] loopback[{}] point[{}]",
    //         i,
    //         i.is_up(),
    //         i.is_running(),
    //         i.is_broadcast(),
    //         i.is_loopback(),
    //         i.is_point_to_point(),
    //     );
    // }
    // }

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
    use scanner::execute::icmp_interface;

    // icmp_interface::send_with_interface();
    tokio::time::sleep(Duration::from_secs(timeout)).await;
}
