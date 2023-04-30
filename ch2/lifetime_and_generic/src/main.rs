use std::ops::Add;
use std::time::Duration;

// lifetime could be elison, see https://rust-lang.github.io/rust-clippy/master/index.html#needless_lifetimes
#[allow(clippy::needless_lifetimes)]
fn add<'a, 'b, T: Add<Output = T> + Copy>(a: &'a T, b: &'b T) -> T {
    *a + *b
}

fn main() {
    let a = 10.0;
    {
        let b = 99.99;
        println!("{a} + {b} = {}", add(&a, &b));
    }
    println!("&1 + &2 = {}", add(&1i32, &2i32));
    println!(
        "&{:?} + &{:?} = {:?}",
        Duration::new(5, 0),
        Duration::new(10, 0),
        add(&Duration::new(5, 0), &Duration::new(10, 0))
    );
    println!("Hello, world!");
}
