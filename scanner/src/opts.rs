use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use clap::{Parser, ValueEnum};
use crate::sockets_iter::SocketIterator;

use super::parse::{parse_hosts, parse_ports};
use super::err::APPError;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum Executes {
    #[default]
    /// send ping to discover hosts reply icmp packets,all packets build by rust purely
    Ping,
    /// use tcp handshake to check tcp ports is open
    Tcp,
    /// send udp packets to remote hosts to check udp opened based on icmp reply
    Udp,
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
    #[arg(short, long, value_parser = parse_ports)]
    pub ports: Option<Arc<Vec<u16>>>,

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
    #[arg(long, default_value_t = 1)]
    pub per_timeout: u64,
}

impl ScanOpts {
    pub fn iter_sockets(&self) -> anyhow::Result<SocketIterator> {
        if let Some(ports) = self.ports.clone() {
            Ok(SocketIterator::new(&*self.hosts.into_iter().map(|x|IpAddr::V4(Ipv4Addr)), &*self.ports))
            // let hosts = (*(self.hosts.clone())).clone();
            // let ports = (*(ports.clone())).clone();
            // let port_hosts = itertools::iprboduct!(ports, hosts);
            // let iter = port_hosts.into_iter()
            //     .map(|(port, host)| SocketAddr::new(IpAddr::V4(host), port));
            // Ok(iter)
        } else {
            Err(APPError::PortIsEmpty.into())
        }
    }
}
