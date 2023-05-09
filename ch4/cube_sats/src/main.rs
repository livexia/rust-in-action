#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

fn check_status(sat: CubeSat) -> StatusMessage {
    let _ = sat;
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat::new(0);
    let sat_b = CubeSat::new(1);
    let sat_c = CubeSat::new(2);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // waiting
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
