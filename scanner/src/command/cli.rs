use std::net::Ipv4Addr;

use clap::{Parser, ValueEnum};

use crate::parse::{parse_hosts, parse_ports};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Executes {
    /// send ping to discover hosts reply icmp packets,all packets build by rust purely
    Ping,
    /// use tcp handshake to check tcp ports is open
    Tcp,
    /// send udp packets to remote hosts to check udp opened based on icmp reply
    Udp,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ScanOpts {
    /// choose a  executor
    #[arg(value_enum)]
    pub execute: Executes,

    /// A list of comma separated CIDRs, IPs, or hosts to be scanned. Example: 1.1.1.1,2.2.2.2/24
    #[arg(value_parser = parse_hosts)]
    pub hosts: Vec<Vec<Ipv4Addr>>,

    /// A list of comma separed ports to be scanned. Example: 80,443,1-100,1-7.
    #[arg(short, long, value_parser = parse_ports)]
    pub ports: Vec<Vec<u16>>,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[arg(long, default_value_t = 4500)]
    pub batch_size: u16,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[arg(long, default_value_t = 1500)]
    pub timeout: u32,

    /// The number of retries before a port is assumed to be closed.
    #[arg(long, default_value_t = 0)]
    pub retries: u8,

    /// The seconds retry interval when retries is set bigger than 0
    #[arg(long, default_value_t = 1)]
    pub retry_interval: u64,
}
