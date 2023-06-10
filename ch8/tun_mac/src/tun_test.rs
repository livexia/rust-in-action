use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    println!("{stream:?}");
}

fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("192.168.42.100:8080")?;
    println!("listen on: 192.168.42.100:8080");

    for (i, stream) in listener.incoming().enumerate() {
        println!("incoming stream: {i}");
        handle_client(stream?);
    }

    Ok(())
}

fn send() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.41.100:8080")?;
    println!("Try to send");

    stream.write(&[1])?;

    let mut buf = [0; 128];
    stream.read(&mut buf)?;
    println!("{:?}", buf);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let _handle = thread::spawn(|| {
        listen().unwrap();
    });
    send()?;
    Ok(())
}
