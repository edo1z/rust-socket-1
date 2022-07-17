mod from_to;
mod network_interface;

use crate::from_to::FromTo;
use crate::network_interface::select_network_interface;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn capture(port: usize) -> Result<()> {
    println!("\nPACKET CAPTURE\n");
    let interface = select_network_interface()?;
    let channel = datalink::channel(&interface, Default::default());
    let (mut _tx, mut rx) = match channel {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("{}", e),
    };
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                let ethertype = packet.get_ethertype();
                match ethertype {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&packet, port);
                    }
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&packet, port);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                panic!("ERROR! {}", e);
            }
        }
    }
}

fn ipv4_handler(ethernet_packet: &EthernetPacket, port: usize) {
    if let Some(ip_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
        let source = ip_packet.get_source().to_string();
        let destination = ip_packet.get_destination().to_string();
        let from_to = FromTo::new(source, destination);
        match ip_packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(ip_packet.payload(), from_to, port);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(ip_packet.payload(), from_to, port);
            }
            _ => {}
        }
    }
}

fn ipv6_handler(ethernet_packet: &EthernetPacket, port: usize) {
    if let Some(ip_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
        let source = ip_packet.get_source().to_string();
        let destination = ip_packet.get_destination().to_string();
        let from_to = FromTo::new(source, destination);
        match ip_packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(ip_packet.payload(), from_to, port);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(ip_packet.payload(), from_to, port);
            }
            _ => {}
        }
    }
}

fn tcp_handler(ip_packet: &[u8], mut from_to: FromTo, port: usize) {
    if let Some(tcp_packet) = TcpPacket::new(ip_packet) {
        let source = tcp_packet.get_source().to_string();
        let destination = tcp_packet.get_destination().to_string();
        if port == 0 || port.to_string() == source || port.to_string() == destination {
            from_to.set_ports(source, destination);
            println!(
                "[tcp] {}\n{:?}",
                from_to.get_from_to(),
                tcp_packet.payload()
            );
        }
    }
}
fn udp_handler(ip_packet: &[u8], mut from_to: FromTo, port: usize) {
    if let Some(udp_packet) = UdpPacket::new(ip_packet) {
        let source = udp_packet.get_source().to_string();
        let destination = udp_packet.get_destination().to_string();
        if port == 0 || port.to_string() == source || port.to_string() == destination {
            from_to.set_ports(source, destination);
            println!(
                "[udp] {}\n{:?}",
                from_to.get_from_to(),
                udp_packet.payload()
            );
        }
    }
}
