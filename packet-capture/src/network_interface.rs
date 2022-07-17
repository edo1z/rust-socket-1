use pnet::datalink::{self, NetworkInterface};
use std::{error::Error, io};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn select_network_interface() -> Result<NetworkInterface> {
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
