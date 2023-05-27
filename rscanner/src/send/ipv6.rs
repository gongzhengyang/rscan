use std::net::Ipv6Addr;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv6::{MutableIpv6Packet};


pub const IPV6_HEADER_LEN: usize = 40;

pub fn build_ipv6_packet(ipv6_packet: &mut MutableIpv6Packet, src_ip: Ipv6Addr, dst_ip: Ipv6Addr, next_protocol: IpNextHeaderProtocol) {
    ipv6_packet.set_source(src_ip);
    ipv6_packet.set_destination(dst_ip);
    ipv6_packet.set_version(6);
    match next_protocol {
        IpNextHeaderProtocols::Tcp => {
            ipv6_packet.set_next_header(IpNextHeaderProtocols::Tcp);
        },
        IpNextHeaderProtocols::Udp => {
            ipv6_packet.set_next_header(IpNextHeaderProtocols::Udp);
        },
        IpNextHeaderProtocols::Icmp => {
            ipv6_packet.set_next_header(IpNextHeaderProtocols::Icmp);
        },
        _ => {},
    }
}
