use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug, Default)]
struct MailBox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64, // MHz
}

impl MailBox {
    fn new() -> Self {
        Self::default()
    }

    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                return Some(self.messages.remove(i));
            }
        }
        None
    }
}

impl CubeSat {
    fn new(id: u64) -> Self {
        Self { id }
    }

    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

impl Copy for CubeSat {}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        Self { id: self.id }
    }
}

impl Copy for StatusMessage {}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn check_status(sat: &CubeSat) -> StatusMessage {
    let _ = sat.id;
    StatusMessage::Ok
}

impl GroundStation {
    fn new(radio_freq: f64) -> Self {
        Self { radio_freq }
    }

    fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, id: u64) -> CubeSat {
        CubeSat::new(id)
    }
}

impl Message {
    fn new(to: &CubeSat, content: String) -> Self {
        Self { to: to.id, content }
    }

    fn read(self) -> String {
        self.content
    }
}

fn fetch_ids() -> Vec<u64> {
    vec![0, 1, 2, 3]
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

    let mut mailbox = MailBox::new();
    let base = GroundStation::new(87.65);

    // Send
    let sat_ids = fetch_ids();

    for id in sat_ids {
        let sat = base.connect(id);
        let msg = Message::new(&sat, format!("Hello: {id}"));
        base.send(&mut mailbox, msg)
    }

    // Recv
    let sat_ids = fetch_ids();

    for id in sat_ids {
        let sat = base.connect(id);
        let msg = sat.recv(&mut mailbox);
        if let Some(msg) = msg {
            println!("{:?}: {}", sat, msg.read());
        }
    }

    let base = Rc::new(RefCell::new(GroundStation::new(87.65)));

    println!("base: {:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
        // After this base_2 droped
    }

    // base_2 droped, so we can view what'is inside or base
    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    // base_3 is still alive, so we can not view what's inside base
    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);
}
