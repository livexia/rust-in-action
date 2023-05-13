fn main() {
    let bits = 42.42f32.to_bits();
    println!("42.42: {bits:032b}");
    let sign_bit = bits >> 31;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = (bits << 9) >> 9;
    println!("Sign bit: {sign_bit:01b}");
    println!("Exponent: {exponent:08b}");
    println!("Mantissa: {mantissa:023b}");
}
