use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use pnet::datalink::{Channel, DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

use crate::interfaces::{get_interface_ipv4, interface_normal_running};
use pnet::datalink::Config;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::{MutablePacket, Packet};

use crate::opts::ScanOpts;

fn send_arp_packets(interface: NetworkInterface, target_ips: Vec<Ipv4Addr>) -> anyhow::Result<()> {
    if !interface_normal_running(&interface) {
        return Ok(());
    }
    let source_ip = get_interface_ipv4(&interface);
    if source_ip.is_none() {
        return Ok(());
    }
    let source_ip = source_ip.unwrap();
    let mac = interface.mac.unwrap();
    let (mut sender, _) = get_sender_receiver(&interface);
    for target_ip in target_ips {
        let mut ethernet_buffer = [0u8; 42];
        let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

        ethernet_packet.set_destination(MacAddr::broadcast());
        ethernet_packet.set_source(mac);
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        let mut arp_buffer = [0u8; 28];
        let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

        arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_packet.set_protocol_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(ArpOperations::Request);
        arp_packet.set_sender_hw_addr(mac);
        arp_packet.set_sender_proto_addr(source_ip);
        arp_packet.set_target_hw_addr(MacAddr::zero());
        arp_packet.set_target_proto_addr(target_ip);

        ethernet_packet.set_payload(arp_packet.packet_mut());

        sender
            .send_to(ethernet_packet.packet(), None)
            .unwrap()
            .unwrap();
        println!("Sent ARP request");
    }
    Ok(())
}

fn receive_packets(interface: NetworkInterface) {
    let (_, mut receiver) = get_sender_receiver(&interface);

    while let buf = receiver.next().unwrap() {
        let arp = ArpPacket::new(&buf[MutableEthernetPacket::minimum_packet_size()..]).unwrap();
        if arp.get_operation() == ArpOperations::Reply {
            println!("Received reply");
            let hw_addr = arp.get_sender_hw_addr();
            println!("{hw_addr:?}");
        }
        // if arp.get_sender_proto_addr() == target_ip
        //     && arp.get_target_hw_addr() == interface.mac.unwrap()
        // {
        //
        // }
        // // let hw_addr = arp.get_sender_hw_addr();
        // // println!("{hw_addr:?}");
    }
}

fn get_sender_receiver(
    interface: &NetworkInterface,
) -> (Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>) {
    let config = Config {
        read_timeout: Some(Duration::from_secs(1)),
        ..Default::default()
    };
    match pnet::datalink::channel(&interface, config) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    }
}

pub async fn scan(scan_opts: ScanOpts) -> anyhow::Result<()> {
    for interface in pnet::datalink::interfaces() {
        let target_ips = (*scan_opts.hosts).clone();
        let interface_cloned = interface.clone();
        tokio::spawn(async move { send_arp_packets(interface_cloned, target_ips) });
        tokio::spawn(async move { receive_packets(interface) });
    }
    Ok(())
}
