use std::net::Ipv4Addr;

use cached::proc_macro::cached;
use pnet::datalink::{Channel, MacAddr, NetworkInterface};
use pnet::packet::{
    ethernet::{EtherTypes, MutableEthernetPacket},
    icmp::echo_request::MutableEchoRequestPacket,
    ip::IpNextHeaderProtocols,
    ipv4::MutableIpv4Packet,
    MutablePacket, Packet,
};
use snafu::{OptionExt, ResultExt};

use crate::err::{CommonIoSnafu, OptionEmptySnafu, Result};
use crate::interfaces::{self, interface_normal_running};

use super::common;

#[cached(time = 60)]
pub fn running_interface_with_ip() -> Vec<(NetworkInterface, Ipv4Addr)> {
    pnet::datalink::interfaces()
        .into_iter()
        .filter(interface_normal_running)
        .filter_map(|interface| interfaces::get_interface_ipv4(&interface).map(|x| (interface, x)))
        .collect()
}

pub fn send_with_interface(target_ip: Ipv4Addr) -> Result<()> {
    tracing::debug!("{target_ip} send by specific interface");
    for (interface, source_ip) in running_interface_with_ip() {
        let mut header = [0u8; common::ICMP_LEN];
        let mut icmp_packet =
            MutableEchoRequestPacket::new(&mut header).context(OptionEmptySnafu)?;
        common::set_icmp_send_packet(&mut icmp_packet);

        let mut ipv4_header = [0u8; common::IPV4_LEN];
        let mut ipv4_packet = MutableIpv4Packet::new(&mut ipv4_header).context(OptionEmptySnafu)?;
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
        let mut ethernet_packet =
            MutableEthernetPacket::new(&mut ethernet_buffer).context(OptionEmptySnafu)?;

        ethernet_packet.set_destination(MacAddr::broadcast());
        ethernet_packet.set_source(interface.mac.context(OptionEmptySnafu)?);
        ethernet_packet.set_ethertype(EtherTypes::Ipv4);
        ethernet_packet.set_payload(ipv4_packet.packet_mut());

        let (mut sender, _) = match pnet::datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unknown channel type"),
            Err(e) => panic!("Error happened {}", e),
        };
        sender
            .send_to(ethernet_packet.packet(), Some(interface))
            .context(OptionEmptySnafu)?
            .context(CommonIoSnafu)?;
    }
    Ok(())
}
