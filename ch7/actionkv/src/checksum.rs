use crc::{Crc, CRC_32_ISO_HDLC};

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

pub fn parity_bit(bytes: &[u8]) -> u8 {
    (bytes.iter().fold(0, |ones, b| {
        if b.count_ones() % 2 == 0 {
            ones
        } else {
            (ones + 1) % 2
        }
    }) % 2
        == 0) as u8
}

pub fn crc32_checksum(bytes: &[u8]) -> u32 {
    CRC.checksum(bytes)
}

#[cfg(test)]
mod checksum_test {
    use super::*;

    #[test]
    fn parity_bit_test() {
        assert_eq!(parity_bit(b"abc"), 1);
        assert_eq!(parity_bit(b"abcd"), 0);
    }

    #[test]
    fn crc32_checksum_test() {
        // compare with crc32 from python zlib
        assert_eq!(crc32_checksum(b"abc"), 891568578);
        assert_eq!(crc32_checksum(b"abcd"), 3984772369);
    }
}
