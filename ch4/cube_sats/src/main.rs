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

struct GroundStation;

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

fn check_status(sat: &CubeSat) -> StatusMessage {
    let _ = sat.id;
    StatusMessage::Ok
}

impl GroundStation {
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
    let base = GroundStation {};

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
}
