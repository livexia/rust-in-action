#![feature(strict_provenance)]
use std::println;

static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new("b");
    let boxed_int = Box::new(456);
    let fn_int = noop();

    let global_ptr = &GLOBAL as *const i32;
    let local_str_ptr = local_str as *const str;
    let local_int_ptr = &local_int as *const i32;
    let boxed_str_ptr = Box::into_raw(boxed_str);
    let boxed_int_ptr = Box::into_raw(boxed_int);

    println!("GLOBAL:    {:p}", global_ptr);
    println!("local_str: {:p}", local_str_ptr);
    println!("local_int: {:p}", &local_int_ptr);
    println!("boxed_str: {:p}", boxed_str_ptr);
    println!("boxed_int: {:p}", boxed_int_ptr);
    println!("fn_int:    {:p}", fn_int);

    let max_ptr = global_ptr
        .expose_addr()
        .max(local_str_ptr.expose_addr())
        .max(local_int_ptr.expose_addr())
        .max(boxed_str_ptr.expose_addr())
        .max(boxed_int_ptr.expose_addr())
        .max(fn_int.expose_addr());

    println!("max ptr:    0x{:0x}", max_ptr);
    println!("as decimal: {}", max_ptr);
    println!("half of the address space: {}", 2usize.pow(63))
}
