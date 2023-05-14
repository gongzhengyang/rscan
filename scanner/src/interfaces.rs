use pnet::datalink::{Channel, MacAddr, NetworkInterface};
use std::net::{IpAddr, Ipv4Addr};

pub fn get_interface_ipv4(interface: NetworkInterface) -> Ipv4Addr {
    interface
        .ips
        .iter()
        .find(|ip| ip.is_ipv4())
        .map(|ip| match ip.ip() {
            IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        })
        .unwrap()
}

pub fn get_ip_by_name(interface_name: &str) -> NetworkInterface {
    pnet::datalink::interfaces()
        .into_iter()
        .find(|interface| interface.name == interface_name)
        .unwrap()
}
