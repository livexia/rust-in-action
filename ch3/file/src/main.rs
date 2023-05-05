use rand::random;

static mut ERROR: isize = 0;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: String) -> Self {
        File {
            name,
            data: Vec::new(),
        }
    }

    fn open(&mut self) -> bool {
        true
    }

    fn close(&mut self) -> bool {
        // testing with global variables to propagate error
        unsafe {
            if ERROR != 0 {
                panic!("An error has occurred!")
            }
        }

        true
    }

    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        if random() && random() && random() {
            // testing with global variables to propagate error
            unsafe {
                ERROR = 1;
            }
            return 0;
        }
        let mut temp = self.data.clone();
        let read_length = temp.len();

        save_to.reserve(read_length);
        save_to.append(&mut temp);
        read_length
    }

    fn write(&mut self, input: &[u8]) -> usize {
        self.data.extend_from_slice(input);
        input.len()
    }
}

fn main() {
    let mut file = File::new("2.txt".to_string());

    let mut buffer = vec![];

    file.open();
    file.write(&[114, 117, 115, 116, 31]);
    file.read(&mut buffer);
    file.close();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} byte long", file.name, file.data.len());
    println!("{}", text);
}
