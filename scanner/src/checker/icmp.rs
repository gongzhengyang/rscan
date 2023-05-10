use std::{
    time::{Duration, Instant},
};
use std::net::IpAddr;

use pnet::packet::{
    icmp::{
        echo_reply::EchoReplyPacket,
        echo_request::{IcmpCodes, MutableEchoRequestPacket},
        IcmpTypes,
    },
    ip::IpNextHeaderProtocols,
    Packet,
};

pub fn ping_ip(target_ip: IpAddr) -> anyhow::Result<()> {
    let channel_type = pnet_transport::TransportChannelType::Layer4(
        pnet_transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp),
    );
    let (mut tx, mut rx) = pnet_transport::transport_channel(4096, channel_type)?;

    let mut iter = pnet_transport::icmp_packet_iter(&mut rx);

    loop {
        let icmp_packet = build_icmp_packet();
        let start_time = Instant::now();
        tx.send_to(icmp_packet, target_ip)?;

        match iter.next() {
            // 匹配 EchoReplyPacket 数据包
            Ok((packet, addr)) => match EchoReplyPacket::new(packet.packet()) {
                Some(echo_reply) => {
                    if packet.get_icmp_type() == IcmpTypes::EchoReply {
                        println!(
                            "ICMP EchoReply received from {:?}: {:?} , Time:{:?}",
                            addr,
                            packet.get_icmp_type(),
                            start_time.elapsed()
                        );
                    } else {
                        println!(
                            "ICMP type other than reply (0) received from {:?}: {:?}",
                            addr,
                            packet.get_icmp_type()
                        );
                    }
                }
                None => {}
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
        }

        std::thread::sleep(Duration::from_millis(500));
    }
}

fn build_icmp_packet() -> MutableEchoRequestPacket {
    let mut icmp_packet = MutableEchoRequestPacket::new(&mut *[0u8; 64]).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(rand::random::<u16>());
    icmp_packet.set_sequence_number(1);
    let checksum = pnet::packet::util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);

    icmp_packet
}
