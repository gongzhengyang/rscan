use std::net::Ipv4Addr;

use pnet::datalink::{Channel, MacAddr};
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::{
    icmp::echo_request::MutableEchoRequestPacket, ip::IpNextHeaderProtocols,
    ipv4::MutableIpv4Packet,
};
use pnet::packet::{MutablePacket, Packet};
use snafu::OptionExt;
use crate::err::OptionEmptySnafu;

use crate::interfaces;
use crate::interfaces::interface_normal_running;
use crate::err::Result;
use super::common;

pub fn send_with_interface(target_ip: Ipv4Addr) -> Result<()> {
    tracing::debug!("{target_ip} send by specific interface");
    for interface in pnet::datalink::interfaces() {
        if !interface_normal_running(&interface) {
            continue;
        }
        if let Some(source_ip) = interfaces::get_interface_ipv4(&interface) {
            let mut header = [0u8; common::ICMP_LEN];
            let mut icmp_packet = MutableEchoRequestPacket::new(&mut header).context(OptionEmptySnafu)?;
            common::modify_icmp_packet(&mut icmp_packet);

            let mut ipv4_header = [0u8; common::IPV4_LEN];
            let mut ipv4_packet = MutableIpv4Packet::new(&mut ipv4_header).unwrap();
            ipv4_packet.set_version(4);
            ipv4_packet.set_header_length(5);
            ipv4_packet.set_dscp(4);
            ipv4_packet.set_ecn(1);
            ipv4_packet.set_ttl(64);
            ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
            ipv4_packet.set_source(source_ip);
            ipv4_packet.set_destination(target_ip);
            ipv4_packet.set_total_length(common::IPV4_LEN as u16);
            ipv4_packet.set_payload(icmp_packet.packet_mut());
            ipv4_packet.set_checksum(pnet::packet::ipv4::checksum(&ipv4_packet.to_immutable()));

            let mut ethernet_buffer = [0u8; common::ETHERNET_LEN];
            let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

            ethernet_packet.set_destination(MacAddr::broadcast());
            ethernet_packet.set_source(interface.mac.unwrap());
            ethernet_packet.set_ethertype(EtherTypes::Ipv4);
            ethernet_packet.set_payload(ipv4_packet.packet_mut());

            let (mut sender, _) = match pnet::datalink::channel(&interface, Default::default()) {
                Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unknown channel type"),
                Err(e) => panic!("Error happened {}", e),
            };
            sender
                .send_to(ethernet_packet.packet(), Some(interface))
                .unwrap()
                .unwrap();
        }
    }
    Ok(())
}
