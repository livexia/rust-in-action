use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Result, Seek, SeekFrom};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use crc::{Crc, CRC_32_ISO_HDLC};
use serde_derive::{Deserialize, Serialize};

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
        eprintln!("Load store!");

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

    pub fn get(&self, key: &str) -> Result<Option<&[u8]>> {
        eprintln!("Get key: {key}");
        if let Some(&position) = self.index.get(key.as_bytes()) {
            let mut f = BufReader::new(&self.f);
            f.seek(SeekFrom::Start(position))?;
            let kv = match ActionKV::process_record(&mut f) {
                Ok(kv) => kv,
                Err(err) => {
                    if let io::ErrorKind::UnexpectedEof = err.kind() {}
                    return Err(err);
                }
            };
            eprintln!("kv: {:?}", kv);
            todo!()
        } else {
            Ok(None)
        }
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
        eprintln!("Update key: {key}, value: {value}");
        None
    }

    fn process_record<R: Read>(f: &mut R) -> Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let value_len = f.read_u32::<LittleEndian>()?;

        let data_len = key_len + value_len;
        let mut buf = ByteString::with_capacity(data_len as usize);

        f.by_ref().take(data_len as u64).read_to_end(&mut buf)?;

        debug_assert_eq!(buf.len(), data_len as usize);

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let checksum = crc.checksum(&buf);

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
