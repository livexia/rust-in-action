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

#[cfg(test)]
mod checksum_test {
    use super::*;

    #[test]
    fn parity_bit_test() {
        assert_eq!(parity_bit(b"abc"), 1);
        assert_eq!(parity_bit(b"abcd"), 0);
    }
}
