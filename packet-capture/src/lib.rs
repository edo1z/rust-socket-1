use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
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
                println!("{}", packet.get_ethertype());
            }
            Err(e) => {
                panic!("ERROR! {}", e);
            }
        }
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
