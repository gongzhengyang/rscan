use std::net::Ipv4Addr;
use std::sync::Arc;

use clap::{Parser, ValueEnum};

use crate::setting::sockets_iter::SocketIterator;

use super::parse::{parse_hosts, parse_ports};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum ScanType {
    #[default]
    /// send icmp to discover hosts reply icmp packets,all packets build by rust purely
    /// use can be `sudo rscan icmp 1.1.1.1/24`
    Icmp,
    /// use tcp handshake to check tcp ports is open, basic can be
    /// `sudo rscan tcp 10.30.6.151/24 --ports 80-1000 --timeout 10`
    Tcp,
    /// send udp packets to remote hosts to check udp opened based on icmp reply
    Udp,
    /// send arp packets
    Arp,
    /// config show
    Show,
}

#[derive(Parser, Debug, Default, Clone)]
#[command(author, version, about, long_about = None)]
pub struct ScanOpts {
    /// choose scan type
    #[arg(value_enum)]
    pub execute: ScanType,

    /// A list of comma separated CIDRs, IPs, or hosts to be scanned.
    /// Eg: 1.1.1.1,2.2.2.2/24
    #[arg(value_parser = parse_hosts)]
    pub hosts: Arc<Vec<Ipv4Addr>>,

    /// A list of comma separated ports to be scanned. Example: 80,443,1-100,1-7.
    #[arg(short, long, value_parser = parse_ports, default_value = "")]
    pub ports: Arc<Vec<u16>>,

    /// The batch size for port scanning.
    /// It increases or slows the speed of scanning.
    /// Depends on the open file limit of your OS.
    /// If you do 65535, it will do every port at the same time.
    /// Although, your OS may not support this.
    #[arg(long, default_value_t = 1024)]
    pub batch_size: usize,

    /// The global timeout in seconds before a port is assumed to be closed.
    #[arg(long, default_value_t = 10)]
    pub timeout: u64,

    /// The number of retries for sending icmp,tcp,udp packets to remote host
    #[arg(long, default_value_t = 3)]
    pub retry: u64,

    /// The seconds retry interval when retry is set bigger than 0
    #[arg(long, default_value_t = 3)]
    pub retry_interval: u64,

    /// save ipaddr results into file at `tmp/2023-12-12--16:25:55.txt` if the args is None
    #[arg(long)]
    pub filepath: Option<String>,
}

impl ScanOpts {
    pub fn iter_sockets(&self) -> anyhow::Result<SocketIterator> {
        Ok(SocketIterator::new(&self.hosts, &self.ports))
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, SocketAddr};
    use std::str::FromStr;

    use super::*;

    #[test]
    fn iter_sockets() {
        let ports = vec![1, 2, 3, 4];
        let hosts = ["127.0.0.1", "192.168.0.1"]
            .into_iter()
            .map(|x| Ipv4Addr::from_str(x).unwrap())
            .collect::<Vec<Ipv4Addr>>();
        let scan_opts = ScanOpts {
            execute: ScanType::Tcp,
            hosts: Arc::new(hosts.clone()),
            ports: Arc::new(ports.clone()),
            ..Default::default()
        };
        let mut iter = scan_opts.iter_sockets().unwrap();
        for port in &ports {
            for host in &hosts {
                assert_eq!(iter.next(), Some(SocketAddr::new(IpAddr::V4(*host), *port)));
            }
        }
    }
}
