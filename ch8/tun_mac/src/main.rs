use std::io::Read;

use tun::Device;

// see: https://lib.rs/crates/tun
fn main() {
    let mut config = tun::Configuration::default();
    config
        .address((192, 168, 42, 100))
        .netmask((255, 255, 255, 0))
        .up();

    let mut dev = tun::create(&config).unwrap();
    let mut buf = [0; 4096];
    println!("broadcast: {:?}", dev.address());

    loop {
        let amount = dev.read(&mut buf).unwrap();
        println!("{:?}", &buf[0..amount]);
    }
}

#[cfg(test)]
mod tests {
    use std::net::{TcpListener, TcpStream};

    fn handel_client(stream: TcpStream) {
        println!("{stream:?}");
    }

    #[test]
    fn it_works() -> std::io::Result<()> {
        let listener = TcpListener::bind("192.168.42.100:80")?;

        for stream in listener.incoming() {
            handel_client(stream?);
        }

        Ok(())
    }
}
