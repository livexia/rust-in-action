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
        eprintln!("Open file: {path:?}");
        Ok(Store::new())
    }
}

impl Store {
    fn new() -> Self {
        Self {}
    }

    pub fn load(&mut self) -> Result<()> {
        eprintln!("Load store!");
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&[u8]> {
        eprintln!("Get key: {key}");
        None
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<&[u8]> {
        eprintln!("Insert key: {key}, value: {value}");
        None
    }

    pub fn delete(&mut self, key: &str) -> Option<&[u8]> {
        eprintln!("Delete key: {key}");
        None
    }

    pub fn update(&mut self, key: &str, value: &str) -> Option<&u8> {
        eprintln!("Update ket: {key}, value: {value}");
        None
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        use byteorder::{ByteOrder, LittleEndian};
        let buf = [11, 11, 11, 11, 11, 11];
        for chunk in buf.chunks(2) {
            assert_eq!((11_u16 << 8) + 11, LittleEndian::read_u16(chunk));
        }
    }
}
