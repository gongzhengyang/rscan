use std::net::{
    IpAddr, Ipv4Addr
};
use std::sync::Arc;

use clap::{Parser, ValueEnum};

use crate::err::APPError;
use crate::setting::sockets_iter::SocketIterator;

use super::parse::{parse_hosts, parse_ports};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum Executes {
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
    /// choose a  executor
    #[arg(value_enum)]
    pub execute: Executes,

    /// A list of comma separated CIDRs, IPs, or hosts to be scanned. Example: 1.1.1.1,2.2.2.2/24
    #[arg(value_parser = parse_hosts)]
    pub hosts: Arc<Vec<Ipv4Addr>>,

    /// A list of comma separed ports to be scanned. Example: 80,443,1-100,1-7.
    #[arg(short, long, value_parser = parse_ports, default_value = "")]
    pub ports: Arc<Vec<u16>>,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[arg(long, default_value_t = 64)]
    pub batch_size: usize,

    /// The global timeout in seconds before a port is assumed to be closed.
    #[arg(long, default_value_t = 30)]
    pub timeout: u64,

    /// The number of retries for sending icmp,tcp,udp packets to remote host
    #[arg(long, default_value_t = 0)]
    pub retry: u64,

    /// The seconds retry interval when retry is set bigger than 0
    #[arg(long, default_value_t = 1)]
    pub retry_interval: u64,

    /// every single operation timeout, tcp connect timeout ro udp timeout
    #[arg(long, default_value_t = 3)]
    pub per_timeout: u64,
}

impl ScanOpts {
    pub fn iter_sockets(&self) -> anyhow::Result<SocketIterator> {
        // let ips = self
        //     .hosts
        //     .iter()
        //     .map(|x| IpAddr::V4(*x))
        //     .collect::<Vec<IpAddr>>();
        // let ports = self.ports.clone().ok_or(APPError::PortIsEmpty)?;
        Ok(SocketIterator::new(&self.hosts, &self.ports))
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use crate::execute::arp::scan;
    use std::str::FromStr;
    use super::*;

    #[test]
    fn iter_sockets() {
        let scan_opts = ScanOpts {
            execute: Executes::Tcp,
            hosts: Arc::new(vec![Ipv4Addr::from([127, 0, 0, 1]),
            Ipv4Addr::from([192, 168, 0, 1])]),
            ports: Arc::new(vec![1, 2, 3]),
            ..Default::default()
        };
        let iter = scan_opts.iter_sockets().unwrap();
        for port in [1, 2] {
            for host in ["127.0.0.1", "192.168.0.1"] {}

        }
        assert_eq!(iter.next(), );
        SocketAddr::from_str("127.0.0.1:1").unwrap()
        for socket in  {
            println!("{}", socket);
        }
    }

}