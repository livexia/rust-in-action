//! Simulating fules one step at a time

use rand::prelude::*;
use std::fmt::Display;

/// Represents a file state
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Represents a "file"
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub state: FileState,
    data: Vec<u8>,
}

/// Trait for read
pub trait Read {
    /// Reads file data to save_to.
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

/// Trait for Write
pub trait Write {
    /// Write input to file data.
    fn write(&mut self, input: &[u8]) -> Result<usize, String>;
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    ///
    /// # Example
    ///
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: String) -> Self {
        File {
            name,
            state: FileState::Closed,
            data: Vec::new(),
        }
    }
}

/// Opens a file, set file state to FileState::Open
pub fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        return Err("Permission denied!".to_string());
    }
    if let FileState::Open = f.state {
        return Err("File already open!".to_string());
    }

    f.state = FileState::Open;
    Ok(f)
}

/// Closes a file, set file state to FileState::Closed
pub fn close(mut f: File) -> Result<File, String> {
    if one_in(100_000) {
        return Err("Interrupted by signal!".to_string());
    }
    f.state = FileState::Closed;
    Ok(f)
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

/// Function for random panic, only useful in testing
fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

/// Main funtion for this program and testing.
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

    let mut file = open(file)?;
    file.write(&[114, 117, 115, 116, 31])?;
    let file_length = file.read(&mut buffer)?;
    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", file);
    println!("{}", file); // testing Display
    println!("{} is {} byte long", file.name, file_length);
    println!("{}", text);
    close(file)?;
    // }

    Ok(())
}
