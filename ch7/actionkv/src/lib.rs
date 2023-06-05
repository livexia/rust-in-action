pub mod checksum;
pub mod utils;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde_derive::{Deserialize, Serialize};

use checksum::crc32_checksum;

type ByteString = Vec<u8>;

type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct ActionKV {
    f: File,
    index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> Result<Self> {
        eprintln!("Open file: {path:?}");
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(Self { f, index })
    }

    pub fn load(&mut self) -> Result<()> {
        let mut f = BufReader::new(&self.f);

        loop {
            let position = f.stream_position()?;

            let kv = match ActionKV::process_record(&mut f) {
                Ok(kv) => kv,
                Err(err) => {
                    if let io::ErrorKind::UnexpectedEof = err.kind() {
                        break;
                    }
                    return Err(err);
                }
            };

            self.index.insert(kv.key, position);
        }
        Ok(())
    }

    pub fn get(&self, key: &ByteStr) -> Result<Option<ByteString>> {
        if let Some(&position) = self.index.get(key) {
            let mut f = BufReader::new(&self.f);
            f.seek(SeekFrom::Start(position))?;
            let kv = ActionKV::process_record(&mut f)?;
            Ok(Some(kv.value.to_vec()))
        } else {
            Ok(None)
        }
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> Result<()> {
        let mut f = BufWriter::new(&self.f);

        let key_len = key.len();
        let value_len = value.len();

        let mut buf = ByteString::with_capacity(key_len + value_len);
        buf.extend_from_slice(key);
        buf.extend_from_slice(value);

        let checksum = crc32_checksum(&buf);

        let position = f.stream_position()?;

        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(value_len as u32)?;
        f.write_all(&buf)?;

        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn delete(&mut self, key: &ByteStr) -> Result<()> {
        self.insert(key, b"")?;
        if self.index.remove(key).is_none() {
            Err(Error::new(
                ErrorKind::Other,
                "{key:?} does not exist in index",
            ))
        } else {
            Ok(())
        }
    }

    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> Result<()> {
        self.insert(key, value)
    }

    fn process_record<R: Read>(f: &mut R) -> Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let value_len = f.read_u32::<LittleEndian>()?;

        let data_len = key_len + value_len;
        let mut buf = ByteString::with_capacity(data_len as usize);

        f.by_ref().take(data_len as u64).read_to_end(&mut buf)?;

        debug_assert_eq!(buf.len(), data_len as usize);

        let checksum = crc32_checksum(&buf);

        if saved_checksum != checksum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }

        let value = buf.split_off(key_len as usize);
        let key = buf;

        Ok(KeyValuePair { key, value })
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
