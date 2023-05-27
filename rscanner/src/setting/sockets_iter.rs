use itertools::{iproduct, Product};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

pub struct SocketIterator<'s> {
    product_it: Product<Box<std::slice::Iter<'s, u16>>, Box<std::slice::Iter<'s, Ipv4Addr>>>,
}

impl<'s> SocketIterator<'s> {
    pub fn new(ips: &'s Arc<Vec<Ipv4Addr>>, ports: &'s Arc<Vec<u16>>) -> Self {
        let ports_it = Box::new(ports.iter());
        let ips_it = Box::new(ips.iter());
        Self {
            product_it: iproduct!(ports_it, ips_it),
        }
    }
}

impl<'s> Iterator for SocketIterator<'s> {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        self.product_it
            .next()
            .map(|(port, ip)| SocketAddr::new(IpAddr::V4(*ip), *port))
    }
}
