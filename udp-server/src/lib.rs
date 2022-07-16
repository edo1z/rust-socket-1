use std::error;
use std::net::UdpSocket;
use std::str;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn serve(address: &str) -> Result<()> {
    let server_socket = UdpSocket::bind(address)?;
    loop {
        let mut buf = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buf)?;
        println!("Handling data from {}", src);
        println!("{}", str::from_utf8(&buf[..size])?);
        server_socket.send_to(&buf, src)?;
    }
}
