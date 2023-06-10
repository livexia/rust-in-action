use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread::{self, sleep};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("{stream:?}");
    let mut buf = [0; 4096];
    let size = stream.read(&mut buf)?;

    println!("recv data: \n{:?}", &buf[..size]);
    Ok(())
}

fn listen() -> std::io::Result<()> {
    let url = "0.0.0.0:8080";
    let listener = TcpListener::bind(url)?;
    println!("listen on: {url}");

    for (i, stream) in listener.incoming().enumerate() {
        println!("incoming stream: {i}");
        handle_client(stream?)?;
    }

    Ok(())
}

fn send() -> std::io::Result<()> {
    println!("Try to send");
    let mut stream = TcpStream::connect("192.168.42.100:8080")?;

    stream.write(&[1; 8])?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let _handle = thread::spawn(|| {
        listen().unwrap();
    });
    println!("- {:?}", send());
    sleep(Duration::from_secs(30));
    Ok(())
}
