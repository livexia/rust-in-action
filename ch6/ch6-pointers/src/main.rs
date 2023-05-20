use std::println;

static B: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
static C: [u8; 10] = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;
    let p_b = u64::from_str_radix(format!("{b:p}").strip_prefix("0x").unwrap(), 16).unwrap();
    let p_c = u64::from_str_radix(format!("{c:p}").strip_prefix("0x").unwrap(), 16).unwrap();
    println!("Hello, pointers!, a = {a}, b -> {b:p}, c -> {c:p}");
    println!("{p_b:0x}, p_c = {p_c:0x}, p_c - p_b = {}", p_c - p_b);
}
