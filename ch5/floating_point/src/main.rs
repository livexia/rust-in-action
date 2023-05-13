fn main() {
    let bits = 42.42f32.to_bits();
    const BIAS: i32 = 127;
    const RADIX: f32 = 2.0;
    println!("42.42: {bits:032b}");
    let sign_bit = bits >> 31;
    // unary minus has lower precedence than method calls
    let sign = (-1i32).pow(sign_bit) as f32;
    let exponent_ = (bits >> 23) & 0xff;
    let exponent = (exponent_ as i32) - BIAS;
    let mantissa_ = (bits << 9) >> 9;
    let mantissa = decode_mantissa(mantissa_);
    let result = sign * mantissa * RADIX.powi(exponent);
    println!("Sign bit: {sign_bit:01b}");
    println!("Exponent bit: {exponent_:08b}");
    println!("Mantissa bit: {mantissa_:023b}");
    println!("---- Decode ----");
    println!("Sign: {sign}");
    println!("Exponent: {exponent}");
    println!("Mantissa: {mantissa}");
    println!("Result: {}", result);
    assert_eq!(42.42, result);
}

/// nonspecial cases
fn decode_mantissa(mantissa_bit: u32) -> f32 {
    // implicit 24th bit
    let mut mantissa = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = mantissa_bit & mask;
        if one_at_bit_i != 0 {
            let weight = 2f32.powf(i as f32 - 23.0);
            mantissa += weight;
        }
    }
    mantissa
}
