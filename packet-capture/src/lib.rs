use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    Packet,
};
use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn capture() -> Result<()> {
    println!("\nPACKET CAPTURE\n");
    let interface = select_network_interface()?;
    let channel = datalink::channel(&interface, Default::default());
    let (mut tx, mut rx) = match channel {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("{}", e),
    };
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                println!("{:?}", packet);
                let ethertype = packet.get_ethertype();
                println!("{}", ethertype);
                match ethertype {
                    EtherTypes::Ipv4 => {
                        ipv4_handler(&packet);
                    }
                    EtherTypes::Ipv6 => {
                        ipv6_handler(&packet);
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

fn ipv4_handler(ethernet_packet: &EthernetPacket) {
    let payload = ethernet_packet.payload();
    if let Some(packet) = Ipv4Packet::new(payload) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(packet.payload());
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(packet.payload());
            }
            _ => {}
        }
    }
}

fn ipv6_handler(ethernet_packet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet_packet.payload()) {
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(packet.payload());
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(packet.payload());
            }
            _ => {}
        }
    }
}

fn tcp_handler(packet: &[u8]) {
    println!("tcp {:?}", packet);
}
fn udp_handler(packet: &[u8]) {
    println!("udp {:?}", packet);
}

fn select_network_interface() -> Result<NetworkInterface> {
    let interfaces = datalink::interfaces();
    println!("{} Network Interfaces", interfaces.len());
    for ni in &interfaces {
        println!("--------------------");
        println!("{}: {:?}", ni.index, ni.name);
    }
    println!("--------------------");
    println!(
        "\nPlease choice Network Interface number. [1-{}]",
        interfaces.len()
    );
    let mut interface_index = String::new();
    io::stdin().read_line(&mut interface_index)?;

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.index.to_string() == interface_index.trim())
        .unwrap();

    println!("You choiced {}.", interface.name);
    Ok(interface)
}
