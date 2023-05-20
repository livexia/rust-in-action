use std::println;

static B: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
static C: [u8; 11] = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 0];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;
    let p_b = u64::from_str_radix(format!("{b:p}").strip_prefix("0x").unwrap(), 16).unwrap();
    let p_c = u64::from_str_radix(format!("{c:p}").strip_prefix("0x").unwrap(), 16).unwrap();
    println!("Hello, pointers!, a = {a}, b -> {b:p}, c -> {c:p}");
    println!("p_b = {p_b:0x}, p_c = {p_c:0x}, p_c - p_b = {}", p_c - p_b);

    let d = Box::new(C);
    println!("The type of d is Box<[u8]>: d -> {d:p}");
    println!("&d -> {:p}", &d);
    println!("size of d: {} bytes", std::mem::size_of_val(&d));
    let e = Box::leak(d);
    println!("Using Box::leak() get the &mut T: e -> {e:p}");
    println!("&e -> {:p}", &e);
    println!("&e[0] -> {:p}", &e[0]);
    println!("&e[1] -> {:p}", &e[1]);
    e[10] = 100;
    println!("After change e[10] to 100: {:?}", e);
    println!("C is still unchanged: {:?}", C);
}
