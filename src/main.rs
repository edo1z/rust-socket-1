use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        let mut port: usize = 0;
        if args.len() > 1 {
            port = match &args[1].trim().parse::<usize>() {
                Ok(port) => *port,
                Err(_e) => 0,
            }
        }
        let _ = packet_capture::capture(port);
        std::process::exit(0);
    }
    let protocol: &str = &args[1]; // tcp | udp
    let role: &str = &args[2]; // server | client
    let address: &str = &args[3]; // addr:port
    println!("protocol:{} role:{} address:{}", protocol, role, address);
    match protocol {
        "tcp" => match role {
            "server" => tcp_server::serve(address)?,
            "client" => tcp_client::connect(address)?,
            _ => (),
        },
        "udp" => match role {
            "server" => udp_server::serve(address)?,
            "client" => udp_client::communicate(address)?,
            _ => (),
        },
        _ => (),
    };
    Ok(())
}
