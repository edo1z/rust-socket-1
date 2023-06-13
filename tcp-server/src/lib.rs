use std::error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn serve(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Listening on {}", address);
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || handler(stream).unwrap_or_else(|error| println!("{:?}", error)));
    }
}

fn handler(mut stream: TcpStream) -> Result<()> {
    println!("Handling from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            println!("Connection closed.");
            return Ok(());
        }
        println!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
