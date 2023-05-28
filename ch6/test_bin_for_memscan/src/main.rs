use std::{thread::sleep, time::Duration};

fn main() {
    let a: u32 = 1235;
    let b: u64 = 14567;
    let c: usize = 12999;

    let mut i = 0;
    loop {
        i += 1;
        println!("Try {i} times");
        assert_eq!(a, 1235);
        assert_eq!(b, 14567);
        assert_eq!(c, 12999);
        sleep(Duration::new(5, 0));
    }
}
