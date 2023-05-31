use std::error::Error;
use std::path::Path;
use std::result;

#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub struct ActionKV {}

pub struct Vault {}

impl ActionKV {
    pub fn open(path: &Path) -> Result<Vault> {
        todo!()
    }
}

impl Vault {
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
        todo!()
    }
}
