use std::collections::HashMap;
use std::io::{self, Write};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::ActionKV;
use crate::ByteString;
use clap::{Command, FromArgMatches, Parser, Subcommand};

type Cache = HashMap<ByteString, u64>;

const INDEX_KEY: &str = "+index+";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// FILE for ActionKV
    #[arg(value_name = "FILE")]
    fname: PathBuf,

    /// Operation commands
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// Retrieves the value at key from the store
    Get { key: String },
    /// Adds a key-value pair to the store
    Insert { key: String, value: String },
    /// Removes a key-value pair from the store
    Delete { key: String },
    /// Replaces an old value with a new one
    Update { key: String, value: String },
    /// Retrieves the value as UTF8 String at key from the store
    Show { key: String },
}

impl Subcommands {
    fn execute(&self, store: &mut ActionKV, disk_index: bool) {
        let mut modified = true;
        if disk_index {
            if let Err(err) = read_index_from_disk(store) {
                eprintln!("{err}");
            }
        }
        match self {
            Subcommands::Get { key } | Subcommands::Show { key } => {
                modified = false;
                match store.get(key.as_bytes()) {
                    Ok(value) => match value {
                        None => println!("None"),
                        Some(value) => {
                            if let Subcommands::Get { .. } = self {
                                println!("{:?}: {:?}", key.as_bytes(), value)
                            } else {
                                println!("{key:?}: {:?}", String::from_utf8_lossy(&value))
                            }
                        }
                    },
                    Err(err) => eprintln!("{err:?}"),
                }
            }
            Subcommands::Insert { key, value } => {
                match store.insert(key.as_bytes(), value.as_bytes()) {
                    Ok(_) => println!("Insert {key:?} {value:?}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
            Subcommands::Delete { key } => match store.delete(key.as_bytes()) {
                Ok(_) => println!("Delete {key:?}"),
                Err(err) => eprintln!("{err}"),
            },
            Subcommands::Update { key, value } => {
                match store.update(key.as_bytes(), value.as_bytes()) {
                    Ok(_) => println!("Update {key:?} {value:?}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
        if disk_index && modified {
            if let Err(err) = write_index_to_disk(store) {
                eprintln!("{err}");
            } else {
                eprintln!("Write index to disk");
            }
        }
    }
}

fn read_index_from_disk(store: &mut ActionKV) -> Result<(), std::io::Error> {
    match store.get(INDEX_KEY.as_bytes()) {
        Ok(value) => {
            if let Some(index_as_bytes) = value {
                match bincode::deserialize::<Cache>(&index_as_bytes) {
                    Ok(index) => {
                        store.index = index;
                        // after first time read from disk,
                        // next time need to insert INDEX_KEY back to the index,
                        // because the index from disk does not contain the INDEX_KEY
                        // could append the index from disk to the index with more time complexity
                        store.insert(INDEX_KEY.as_bytes(), &index_as_bytes)
                    }
                    Err(err) => Err(Error::new(ErrorKind::Other, err)),
                }
            } else {
                Err(Error::new(ErrorKind::Other, "index is not on the memory"))
            }
        }
        Err(err) => Err(err),
    }
}

/// Write index to disk, but index does not contain the index
/// if using akv_mem to add key-value, but will not update the index from the disk
/// so next time using akv_disk will not get the updated value.
fn write_index_to_disk(store: &mut ActionKV) -> Result<(), std::io::Error> {
    // remove index's index from index first to avoid recursion
    store.index.remove(INDEX_KEY.as_bytes());
    match bincode::serialize(&store.index) {
        Ok(index_as_bytes) => {
            // clear current index first
            store.index.clear();
            match store.insert(INDEX_KEY.as_bytes(), &index_as_bytes) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(Error::new(ErrorKind::Other, err)),
    }
}

pub fn run(disk_index: bool) {
    let args = Cli::parse();

    let path = args.fname;
    let mut store = ActionKV::open(&path).expect("unable to open file");
    store.load().expect("unable to load data");

    // when using akv_disk first thing to do is update the disk index
    // because some change may not write to the disk
    if disk_index {
        if let Err(err) = write_index_to_disk(&mut store) {
            eprintln!("{err}");
        } else {
            eprintln!("Updating newest index to the disk")
        }
    }

    match &args.command {
        Some(command) => command.execute(&mut store, disk_index),
        None => {
            let prompt = Command::new("Interactive prompt").no_binary_name(true);
            let mut prompt = Subcommands::augment_subcommands(prompt);
            loop {
                print!("[{:?}]> ", path.file_name().expect("unable to open file"));
                io::stdout().flush().expect("unable to flush stdout");

                let mut buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("unable to readline from stdin");

                if buffer.trim() == "exit" || buffer.trim() == "quit" {
                    return;
                }

                if let Some(raw_args) = shlex::split(&buffer) {
                    match prompt.try_get_matches_from_mut(raw_args.into_iter()) {
                        Ok(matches) => match Subcommands::from_arg_matches(&matches) {
                            Ok(command) => command.execute(&mut store, disk_index),
                            Err(err) => eprintln!("{err}"),
                        },
                        Err(err) => eprintln!("{err}"),
                    }
                } else {
                    eprintln!("Input is not valid shell words")
                }
            }
        }
    }
}
