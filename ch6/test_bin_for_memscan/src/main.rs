use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let a: u32 = u32::MAX;
    let b: u64 = 14567;
    let c: usize = 12999;

    let mut i = 0;
    loop {
        i += 1;
        println!("Try {i} times");
        println!("a address: {:p}, size: {}", &a, size_of::<u32>());
        println!("b address: {:p}, size: {}", &b, size_of::<u64>());
        println!("c address: {:p}, size: {}", &c, size_of::<usize>());
        assert_eq!(a, u32::MAX);
        assert_eq!(b, 14567);
        assert_eq!(c, 12999);
        sleep(Duration::new(5, 0));
    }
}
