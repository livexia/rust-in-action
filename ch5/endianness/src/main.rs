fn main() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
    let machine_endian: u32 = 0xDDCCBBAA;

    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(little_endian) };
    let c: i32 = unsafe { std::mem::transmute(machine_endian) };

    println!("{a} vs {b} vs {c}");
}
