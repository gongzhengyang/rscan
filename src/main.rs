use clap::Parser;
use scanner::command::cli::ScanOpts;

#[tokio::main]
async fn main() {
    let cli = ScanOpts::parse();
    println!("{:?}", cli);
    println!("{:?}", cli.execute);
    println!("{:?}", cli.hosts);
    println!("{:?}", cli.ports.concat());
    // println!("{:?}", cli.ports_range.unwrap());
    println!("{}", cli.batch_size);
    println!("{}", cli.timeout);
    println!("{}", cli.retries);
    println!("{}", cli.retry_interval);
    // match cli.command {
    //     ExecuteSubcommands::Ping => {
    //         println!("execute icmp");
    //     }
    //     _ => {
    //         panic!("invalid protocol")
    //     }
    // }
}
