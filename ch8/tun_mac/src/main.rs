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
    println!("address: {:?}", dev.address());

    loop {
        let amount = dev.read(&mut buf).unwrap();
        println!("{:?}", &buf[0..amount]);
    }
}
