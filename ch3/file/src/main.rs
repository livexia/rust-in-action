use rand::prelude::*;

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

    fn open(self) -> Result<Self, String> {
        if one_in(10_000) {
            return Err("Permission denied".to_string());
        }
        Ok(self)
    }

    fn close(self) -> Result<Self, String> {
        if one_in(100_000) {
            return Err("Interrupted by signal!".to_string());
        }
        Ok(self)
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut temp = self.data.clone();
        let read_length = temp.len();

        save_to.reserve(read_length);
        save_to.append(&mut temp);
        Ok(read_length)
    }

    fn write(&mut self, input: &[u8]) -> Result<usize, String> {
        self.data.extend_from_slice(input);
        Ok(input.len())
    }
}

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

fn main() -> Result<(), String> {
    // for _ in 0..1_000_000 {
    let file = File::new("2.txt".to_string());

    let mut buffer = vec![];

    let mut file = file.open()?;
    file.write(&[114, 117, 115, 116, 31])?;
    let file_length = file.read(&mut buffer)?;
    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", file);
    println!("{} is {} byte long", file.name, file_length);
    println!("{}", text);
    file.close()?;
    // }

    Ok(())
}
