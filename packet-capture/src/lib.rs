use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};
use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn capture() -> Result<()> {
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

struct FromTo {
    from_ip: String,
    from_port: Option<String>,
    to_ip: String,
    to_port: Option<String>,
}
impl FromTo {
    fn new(from_ip: String, to_ip: String) -> Self {
        Self {
            from_ip,
            from_port: None,
            to_ip,
            to_port: None,
        }
    }
    fn set_ports(&mut self, from_port: String, to_port: String) {
        self.from_port = Some(from_port);
        self.to_port = Some(to_port);
    }
    fn get_from(&self) -> String {
        format!(
            "{}:{}",
            self.from_ip,
            if let Some(from_port) = &self.from_port {
                from_port
            } else {
                "?"
            }
        )
    }
    fn get_to(&self) -> String {
        format!(
            "{}:{}",
            self.to_ip,
            if let Some(to_port) = &self.to_port {
                to_port
            } else {
                "?"
            }
        )
    }
    fn get_from_to(&self) -> String {
        format!("{} -> {}", self.get_from(), self.get_to())
    }
}

fn ipv4_handler(ethernet_packet: &EthernetPacket) {
    if let Some(ip_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
        let source = ip_packet.get_source().to_string();
        let destination = ip_packet.get_destination().to_string();
        let from_to = FromTo::new(source, destination);
        match ip_packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(ip_packet.payload(), from_to);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(ip_packet.payload(), from_to);
            }
            _ => {}
        }
    }
}

fn ipv6_handler(ethernet_packet: &EthernetPacket) {
    if let Some(ip_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
        let source = ip_packet.get_source().to_string();
        let destination = ip_packet.get_destination().to_string();
        let from_to = FromTo::new(source, destination);
        match ip_packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(ip_packet.payload(), from_to);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(ip_packet.payload(), from_to);
            }
            _ => {}
        }
    }
}

fn tcp_handler(ip_packet: &[u8], mut from_to: FromTo) {
    if let Some(tcp_packet) = TcpPacket::new(ip_packet) {
        let source = tcp_packet.get_source().to_string();
        let destination = tcp_packet.get_destination().to_string();
        from_to.set_ports(source, destination);
        println!(
            "[tcp] {}\n{:?}",
            from_to.get_from_to(),
            tcp_packet.payload()
        );
    }
}
fn udp_handler(ip_packet: &[u8], mut from_to: FromTo) {
    if let Some(udp_packet) = UdpPacket::new(ip_packet) {
        let source = udp_packet.get_source().to_string();
        let destination = udp_packet.get_destination().to_string();
        from_to.set_ports(source, destination);
        println!(
            "[udp] {}\n{:?}",
            from_to.get_from_to(),
            udp_packet.payload()
        );
    }
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
