fn main() {
    let a: f32 = 42.42;
    // let frankentype: u32 = unsafe { std::mem::transmute(a) };
    let frankentype: u32 = a.to_bits();

    println!("{frankentype}");
    println!("{:032b}", frankentype);

    // let b: f32 = unsafe { std::mem::transmute(frankentype) };
    let b: f32 = f32::from_bits(frankentype);

    println!("{b}");
    assert_eq!(a, b);
}
