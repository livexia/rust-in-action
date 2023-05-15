fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b111111 << 24;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    // m is in 0.5..=0.998
    let m = f32::from_bits(f32_bits);
    // normalized to 0..=0.996
    2.0 * (m - 0.5)
}

fn main() {
    for n in (u8::MIN..=u8::MAX).step_by(32) {
        println!("mock rand for: {n} -> {}", mock_rand(n));
    }
    println!("mock rand for: 255 -> {}", mock_rand(255));
    println!("Hello, world!");
}
