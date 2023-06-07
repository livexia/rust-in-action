use std::error::Error;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

fn main() -> Result<(), Box<dyn Error>> {
    let host = "httpstat.us";
    let port = "80";
    let addr: Vec<SocketAddr> = format!("{host}:{port}").to_socket_addrs()?.collect();

    let mut stream = TcpStream::connect(&addr[..])?;
    // 1.1 will hang
    stream.write_all(b"GET / HTTP/1.0")?;
    stream.write_all(b"\r\n")?;

    stream.write_all(b"Host: ")?;
    stream.write(host.as_bytes())?;
    stream.write_all(b"\r\n\r\n")?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("Response: {response}");

    Ok(())
}
