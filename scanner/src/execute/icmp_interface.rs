use std::net::Ipv4Addr;

use pnet::datalink::{Channel, MacAddr};
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::{
    icmp::echo_request::MutableEchoRequestPacket, ip::IpNextHeaderProtocols,
    ipv4::MutableIpv4Packet,
};
use pnet::packet::{MutablePacket, Packet};

use crate::execute::icmp;
use crate::interfaces;

pub fn send_with_interface(target_ip: Ipv4Addr) {
    let interface = interfaces::get_interface_by_name("enp3s0");
    let (mut sender, mut receiver) = match pnet::datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };
    let mut header = [0u8; 64];
    let mut icmp_packet = MutableEchoRequestPacket::new(&mut header).unwrap();
    icmp::modify_icmp_packet(&mut icmp_packet);

    let mut ipv4_header = [0u8; 96];
    let mut ipv4_packet = MutableIpv4Packet::new(&mut ipv4_header).unwrap();
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(5);
    ipv4_packet.set_dscp(4);
    ipv4_packet.set_ecn(1);
    ipv4_packet.set_ttl(64);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    ipv4_packet.set_source(interfaces::get_interface_ipv4(interface.clone()));
    ipv4_packet.set_destination(target_ip);
    ipv4_packet.set_total_length(96);
    ipv4_packet.set_payload(icmp_packet.packet_mut());
    ipv4_packet.set_checksum(pnet::packet::ipv4::checksum(&ipv4_packet.to_immutable()));

    let mut ethernet_buffer = [0u8; 128];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac.unwrap());
    ethernet_packet.set_ethertype(EtherTypes::Ipv4);
    ethernet_packet.set_payload(ipv4_packet.packet_mut());

    sender
        .send_to(ethernet_packet.packet(), Some(interface))
        .unwrap()
        .unwrap();
}
