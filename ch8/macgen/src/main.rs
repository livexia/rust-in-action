use std::fmt::Display;

use rand::RngCore;

struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let octet = &self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    fn new() -> Self {
        let mut octet: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octet);
        octet[0] |= 0b_0000_0011;
        MacAddress(octet)
    }

    fn is_local(&self) -> bool {
        self.0[0] & 0b_0000_0010 == 0b_0000_0010
    }

    // see: https://docs.rs/macaddr/latest/macaddr/struct.MacAddr8.html#method.is_multicast
    fn is_multicast(&self) -> bool {
        self.0[0] & 0b_0000_0001 == 0b_0000_0001
    }

    // see: https://github.com/rust-in-action/code/issues/7#issuecomment-672859672
    fn is_unicast(&self) -> bool {
        self.0[0] & 1 == 0
    }
}
fn main() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_multicast());
    assert!(!mac.is_unicast());
    println!("mac: {mac}");
}
