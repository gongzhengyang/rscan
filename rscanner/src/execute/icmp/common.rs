use pnet::packet::{
    icmp::{
        echo_request::{IcmpCodes, MutableEchoRequestPacket},
        IcmpTypes,
    },
    ip::IpNextHeaderProtocols,
    Packet,
};
use pnet_transport::{TransportReceiver, TransportSender};

pub const ICMP_LEN: usize = 64;
pub const IPV4_LEN: usize = 96;
pub const ETHERNET_LEN: usize = 128;

pub fn get_transport_channel() -> anyhow::Result<(TransportSender, TransportReceiver)> {
    let channel_type = pnet_transport::TransportChannelType::Layer4(
        pnet_transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp),
    );
    Ok(pnet_transport::transport_channel(4096, channel_type)?)
}

pub fn modify_icmp_packet(icmp_packet: &mut MutableEchoRequestPacket) {
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(rand::random::<u16>());
    icmp_packet.set_sequence_number(1);

    let checksum = pnet::packet::util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);
}
