use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn print_buf(buffer: &[u8], pos: usize) {
    print!("[0x{pos:08x}] ");
    for &byte in buffer {
        match byte {
            0x00 => print!(".  "),
            0xff => print!("## "),
            _ => print!("{byte:02x} "),
        }
    }
    println!()
}

fn main() {
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file: {fname}");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print_buf(&buffer, pos);
        pos += BYTES_PER_LINE;
    }
    let end_position = f
        .stream_position()
        .expect("Unable to get the last read position for file");
    if end_position % (BYTES_PER_LINE as u64) != 0 {
        let mut buffer = Vec::with_capacity(BYTES_PER_LINE);
        if let Ok(_) = f.seek(std::io::SeekFrom::Start(pos as u64)) {
            if let Ok(_) = f.read_to_end(&mut buffer) {
                print_buf(&buffer, pos);
            }
        }
    }
}
