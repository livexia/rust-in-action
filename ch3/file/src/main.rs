//! Simulating fules one step at a time

use rand::prelude::*;
use std::fmt::Display;

/// Represents a file state
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    state: FileState,
    data: Vec<u8>,
}

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

trait Write {
    fn write(&mut self, input: &[u8]) -> Result<usize, String>;
}

impl File {
    fn new(name: String) -> Self {
        File {
            name,
            state: FileState::Closed,
            data: Vec::new(),
        }
    }

    fn open(mut self) -> Result<Self, String> {
        if one_in(10_000) {
            return Err("Permission denied!".to_string());
        }
        if let FileState::Open = self.state {
            return Err("File already open!".to_string());
        }

        self.state = FileState::Open;
        Ok(self)
    }

    fn close(mut self) -> Result<Self, String> {
        if one_in(100_000) {
            return Err("Interrupted by signal!".to_string());
        }
        self.state = FileState::Closed;
        Ok(self)
    }
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if let FileState::Closed = self.state {
            return Err("File must be open for reading first!".to_string());
        }
        let mut temp = self.data.clone();
        let read_length = temp.len();

        save_to.reserve(read_length);
        save_to.append(&mut temp);
        Ok(read_length)
    }
}

impl Write for File {
    fn write(&mut self, input: &[u8]) -> Result<usize, String> {
        self.data.extend_from_slice(input);
        Ok(input.len())
    }
}

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[allow(clippy::read_zero_byte_vec)]
fn main() -> Result<(), String> {
    // for _ in 0..1_000_000 {
    let file = File::new("2.txt".to_string());

    let mut buffer = vec![];

    {
        // test file open
        if file.read(&mut buffer).is_err() {
            println!("Error checking is working");
        }
    }

    let mut file = file.open()?;
    file.write(&[114, 117, 115, 116, 31])?;
    let file_length = file.read(&mut buffer)?;
    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", file);
    println!("{}", file); // testing Display
    println!("{} is {} byte long", file.name, file_length);
    println!("{}", text);
    file.close()?;
    // }

    Ok(())
}
