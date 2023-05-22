static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 109, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: i64 = 42;

    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);

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

    let b_string = decoding();
    // After b_string and c_cow out of scope program is crashing,
    // because with String now b_string has the ownership of B
    // when b_string droped, B is also droped?
    // and modify b_string will not work(realloc)
    // String take ownership of the pointer and manage it
    // see: https://stackoverflow.com/a/52651347
    // and: https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts
    // and: https://stackoverflow.com/q/60007368
    println!("b_string: {b_string}");
    drop(b_string);
    println!("After drop b_string, program crashing");
    println!("&B -> {:p}", &B);
}

use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
fn decoding() -> String {
    let a = 42;

    let b: String;

    let c: Cow<str>;

    let d: Cow<str>;

    // SAFETY: this is unsafe, will crasing program
    // this is bad, should be avoid
    // see: https://doc.rust-lang.org/std/string/struct.String.html#method.from_raw_parts
    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;

        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;

        c = CStr::from_ptr(c_ptr).to_string_lossy();

        // This is very unsafe, reading from a pointer, without knowing the string length
        // will try to reading all contiunous memory as char
        // so the d will be decoding both B and C
        // d = carrytowmlthanksfish
        let d_ptr = &B as *const u8 as *const c_char;

        d = CStr::from_ptr(d_ptr).to_string_lossy();
    }
    println!("a: {a}, b: {b}, c: {c}");
    println!("d: {d}");
    b
}
