use std::net::{IpAddr, Ipv4Addr};

use pnet::datalink::NetworkInterface;

pub fn get_interface_ipv4(interface: &NetworkInterface) -> Option<Ipv4Addr> {
    interface
        .ips
        .iter()
        .find(|ip| ip.is_ipv4())
        .map(|ip| match ip.ip() {
            IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        })
}

pub fn get_interface_by_name(interface_name: &str) -> NetworkInterface {
    pnet::datalink::interfaces()
        .into_iter()
        .find(|interface| interface.name == interface_name)
        .unwrap()
}

pub fn interface_normal_running(interface: &NetworkInterface) -> bool {
    #[cfg(unix)]
    if !interface.is_running() {
        return false;
    }
    if interface.is_loopback() {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn all_interfaces() {
        for interface in pnet::datalink::interfaces() {
            println!("{}", interface.name);
        }
    }
}
