use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread::{self, sleep};
use std::time::Duration;

#[allow(dead_code)]
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("{stream:?}");
    let mut buf = [0; 4096];
    let size = stream.read(&mut buf)?;

    println!("recv data: \n{:?}", &buf[..size]);
    Ok(())
}

#[allow(dead_code)]
fn tcp_listen() -> std::io::Result<()> {
    let url = "0.0.0.0:54321";
    let listener = TcpListener::bind(url)?;
    println!("listen on: {url}");

    for (i, stream) in listener.incoming().enumerate() {
        println!("incoming stream: {i}");
        handle_client(stream?)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn tcp_send() -> std::io::Result<()> {
    println!("Try to send");
    let mut stream = TcpStream::connect("192.168.42.100:54321")?;

    stream.write_all(&[0xff; 8])?;

    Ok(())
}

fn udp_testing() -> std::io::Result<()> {
    let addr = "0.0.0.0:34254";
    let socket = UdpSocket::bind(addr)?;

    let connect_addr = "192.168.42.100:34254";
    socket.connect(connect_addr)?;

    let buf = [0xff; 8];
    socket.send(&buf)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    udp_testing()?;
    // tun_mac will show a packet has been send to it
    // end with 8 0xff
    // also could using wireshark to see the udp packet

    let _handle = thread::spawn(|| {
        tcp_listen().unwrap();
    });
    println!("- {:?}", tcp_send());

    sleep(Duration::from_secs(30));
    Ok(())
}
