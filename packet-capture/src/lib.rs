use pnet::datalink;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn capture() -> Result<()> {
    println!("packet capture");
    let interfaces = datalink::interfaces();
    println!("{} Network Interfaces", interfaces.len());
    for ni in interfaces {
        println!("--------------------");
        println!("{}: {:?}", ni.index, ni.name);
        let mac = match ni.mac {
            Some(mac) => format!("{}", mac),
            None => String::from(""),
        };
        println!("mac: {}", mac);
        println!("ips: {:?}", ni.ips);
        println!("flags {:?}", ni.flags);
    }
    Ok(())
}
