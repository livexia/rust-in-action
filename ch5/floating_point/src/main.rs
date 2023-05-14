const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let n: f32 = 42.42;

    let (sign_, exp_, frac) = to_parts(n);
    let (sign, exp, mant) = decode(sign_, exp_, frac);

    let n_ = from_parts(sign, exp, mant);

    println!("{n} -> {n_}");
    println!("feld     |  as bits | as real number");
    println!("sign     |        {sign_:01b} | {sign}");
    println!("exponent | {exp_:08b} | {exp}");
    println!("mantissa | {frac:023b} | {mant}");
    assert_eq!(n, n_);
}

/// dissect
fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = bits >> 31;
    let exponent = (bits >> 23) & 0xff;
    let fraction = (bits << 9) >> 9;
    (sign, exponent, fraction)
}

/// decode
fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    // unary minus has lower precedence than method calls
    let sign = (-1i32).pow(sign) as f32;

    let exponent = RADIX.powi((exponent as i32) - BIAS);

    // nonspecial cases
    // implicit 24th bit
    let mut mantissa = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let weight = 2f32.powf(i as f32 - 23.0);
            mantissa += weight;
        }
    }
    (sign, exponent, mantissa)
}

// build
fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}
