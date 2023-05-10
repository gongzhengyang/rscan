use command_parser::opts::ScanOpts;

#[tokio::main]
async fn main() {
    let opts = ScanOpts::read();
    println!("{opts:?}");
}
