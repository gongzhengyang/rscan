use itertools::{iproduct, Product};
use std::net::{IpAddr, SocketAddr};

pub struct SocketIterator<'s> {
    product_it:
    Product<Box<std::slice::Iter<'s, u16>>, Box<std::slice::Iter<'s, std::net::IpAddr>>>,
}

impl<'s> SocketIterator<'s> {
    pub fn new(ips: &'s [IpAddr], ports: &'s [u16]) -> Self {
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
        match self.product_it.next() {
            None => None,
            Some((port, ip)) => Some(SocketAddr::new(*ip, *port)),
        }
    }
}
