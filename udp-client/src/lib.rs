use std::error;
use std::net::UdpSocket;
use std::{io, str};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn communicate(address: &str) -> Result<()> {
    let socket = UdpSocket::bind("localhost:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), address)?;

        let mut buf = [0u8; 1024];
        socket.recv_from(&mut buf)?;
        print!("{}", str::from_utf8(&buf)?);
    }
}
