use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use pnet::datalink::{
    Channel, Config, DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface,
};
use pnet::packet::{
    arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket},
    ethernet::{EtherTypes, MutableEthernetPacket},
    MutablePacket, Packet,
};
use snafu::OptionExt;

use crate::err::OptionEmptySnafu;
use crate::interfaces::{get_interface_ipv4, interface_normal_running};
use crate::monitor;
use crate::setting::command::ScanOpts;

async fn send_arp_packets(
    interface: NetworkInterface,
    source_ip: Ipv4Addr,
    target_ips: Vec<Ipv4Addr>,
) -> anyhow::Result<()> {
    let mac = interface.mac.context(OptionEmptySnafu)?;
    let (mut sender, _) = get_sender_receiver(&interface);
    tracing::info!("Sent ARP request with interface: {interface:?}");
    for target_ip in target_ips {
        let ipaddr = IpAddr::V4(target_ip);
        if monitor::is_addr_received(&ipaddr).await {
            continue;
        }
        let mut ethernet_buffer = [0u8; 42];
        let mut ethernet_packet =
            MutableEthernetPacket::new(&mut ethernet_buffer).context(OptionEmptySnafu)?;

        ethernet_packet.set_destination(MacAddr::broadcast());
        ethernet_packet.set_source(mac);
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        let mut arp_buffer = [0u8; 28];
        let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).context(OptionEmptySnafu)?;

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
    }
    Ok(())
}

async fn receive_packets(interface: NetworkInterface) {
    let (_, mut receiver) = get_sender_receiver(&interface);
    loop {
        if let Ok(buf) = receiver.next() {
            let arp = ArpPacket::new(&buf[MutableEthernetPacket::minimum_packet_size()..]).unwrap();
            if arp.get_operation() == ArpOperations::Reply {
                let sender_ipaddr = arp.get_sender_proto_addr();
                let ipaddr = IpAddr::V4(sender_ipaddr);
                if monitor::is_addr_received(&ipaddr).await {
                    continue;
                }
                monitor::add_receive_ipaddr(ipaddr).await;
                println!("rscan|arp|{}|{}|", sender_ipaddr, arp.get_sender_hw_addr());
            }
        }
        tokio::time::sleep(Duration::from_micros(10)).await;
    }
}

fn get_sender_receiver(
    interface: &NetworkInterface,
) -> (Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>) {
    let config = Config {
        read_timeout: Some(Duration::from_secs(1)),
        ..Default::default()
    };
    match pnet::datalink::channel(interface, config) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    }
}

pub async fn scan(scan_opts: ScanOpts) -> anyhow::Result<()> {
    for interface in pnet::datalink::interfaces() {
        if !interface_normal_running(&interface) {
            continue;
        }
        let source_ip = get_interface_ipv4(&interface);
        if source_ip.is_none() {
            continue;
        }
        let source_ip = source_ip.unwrap();
        let target_ips = (*scan_opts.hosts).clone();
        let interface_cloned = interface.clone();
        tokio::spawn(
            async move { send_arp_packets(interface_cloned, source_ip, target_ips).await },
        );
        tokio::spawn(async move { receive_packets(interface).await });
    }
    Ok(())
}
