use std::error::Error;
use std::path::Path;
use std::result;

#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub struct ActionKV {}

pub struct Store {}

impl ActionKV {
    pub fn open(path: &Path) -> Result<Store> {
        todo!()
    }
}

impl Store {
    pub fn load(&mut self) -> Result<()> {
        todo!()
    }

    pub fn get(&self, key: &str) -> Option<&[u8]> {
        todo!()
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<&[u8]> {
        todo!()
    }

    pub fn delete(&mut self, key: &str) -> Option<&[u8]> {
        todo!()
    }

    pub fn update(&mut self, key: &str, value: &str) -> Option<&u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use byteorder::{ByteOrder, LittleEndian};
        let mut buf = [11, 11, 11, 11, 11, 11];
        for chunk in buf.chunks(2) {
            assert_eq!((11_u16 << 8) + 11, LittleEndian::read_u16(chunk));
        }
    }
}
