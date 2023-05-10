#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug, Default)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

type Message = String;

#[derive(Debug, Default)]
struct MailBox {
    messages: Vec<Message>,
}

struct GroundStation;

impl CubeSat {
    fn new(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    fn recv(&mut self) -> Option<String> {
        self.mailbox.messages.pop()
    }
}

fn check_status(sat: &CubeSat) -> StatusMessage {
    let _ = sat.id;
    StatusMessage::Ok
}

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn main() {
    let sat_a = CubeSat::new(0);
    let sat_b = CubeSat::new(1);
    let sat_c = CubeSat::new(2);

    let a_status = check_status(&sat_a);
    let b_status = check_status(&sat_b);
    let c_status = check_status(&sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // waiting
    let a_status = check_status(&sat_a);
    let b_status = check_status(&sat_b);
    let c_status = check_status(&sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let base = GroundStation {};
    let mut sat_a = CubeSat::new(0);
    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, "Hello".to_string());
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    println!("sat_a received: {:?}", msg);
}
