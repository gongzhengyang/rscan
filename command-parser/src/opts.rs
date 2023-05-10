use crate::err::APPError;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

fn parse_range(input: &str) -> anyhow::Result<PortRange> {
    let range = input
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<u16>, std::num::ParseIntError>>()?;
    if let [start, end] = range.as_slice() {
        return Ok(PortRange {
            start: *start,
            end: *end,
        });
    }
    Err(APPError::PortFormatError.into())
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "rscan", setting = structopt::clap::AppSettings::TrailingVarArg)]
/// Fast ping, tcp scan, udp scan write in rust.
/// Inspired by nmap,RustScan
/// Doesn't depend on nmap
/// WARNING Do not use this program against sensitive infrastructure since the
/// specified server may not be able to handle this many socket connections at once.
pub struct ScanOpts {
    /// A list of comma separated CIDRs, IPs, or hosts to be scanned.
    #[structopt(short, long, use_delimiter = true)]
    pub addresses: Vec<String>,

    /// A list of comma separed ports to be scanned. Example: 80,443,8080.
    #[structopt(short, long, use_delimiter = true)]
    pub ports: Option<Vec<u16>>,

    /// A range of ports with format start-end. Example: 1-1000.
    #[structopt(short, long, conflicts_with = "ports", parse(try_from_str = parse_range))]
    pub range: Option<PortRange>,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[structopt(short, long, default_value = "4500")]
    pub batch_size: u16,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[structopt(short, long, default_value = "1500")]
    pub timeout: u32,

    /// The number of retries before a port is assumed to be closed.
    #[structopt(long, default_value = "0")]
    pub retries: u8,

    /// The seconds retry interval when retries is set bigger than 1
    #[structopt(long, default_value = "1")]
    pub retry_interval: u8,
}

impl ScanOpts {
    pub fn read() -> ScanOpts {
        ScanOpts::from_args()
    }
}
