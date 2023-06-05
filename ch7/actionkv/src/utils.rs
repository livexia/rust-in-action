use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::ActionKV;
use crate::ByteString;
use clap::{Command, FromArgMatches, Parser, Subcommand};

type Cache = HashMap<ByteString, u64>;

const INDEX_KEY: &'static str = "+index+";

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
    fn execute(&self, store: &mut ActionKV, index_cache: &mut Option<Cache>) {
        match self {
            Subcommands::Get { key } => match store.get(key.as_bytes()) {
                Ok(value) => println!("{:?}: {:?}", key.as_bytes(), value.unwrap_or(vec![])),
                Err(err) => eprintln!("{err:?}"),
            },
            Subcommands::Show { key } => match store.get(key.as_bytes()) {
                Ok(value) => match value {
                    None => println!("None"),
                    Some(value) => println!("{key:?}: {:?}", String::from_utf8_lossy(&value)),
                },
                Err(err) => eprintln!("{err}"),
            },
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
    }
}

fn get_cache(cache: &mut Cache, store: &ActionKV) -> Result<(), std::io::Error> {
    match store.get(INDEX_KEY.as_bytes()) {
        Ok(value) => {
            if let Some(index_as_bytes) = value {
                match bincode::deserialize::<Cache>(&index_as_bytes) {
                    Ok(index) => *cache = index,
                    Err(_) => {
                        eprintln!("Reading index cache from disk error, Writing current index cache to the disk");
                    }
                }
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn get_with_cache(cache: &mut Cache, key: &ByteString) -> Option<u64> {
    cache.get(key).copied()
}

fn update_cache(cache: &mut Cache, key: ByteString, position: u64) {
    cache.insert(key, position);
}

fn delete_from_cache(cache: &mut Cache, key: &ByteString) {
    cache.remove(key);
}

pub fn run(index_cache: &mut Option<Cache>) {
    let args = Cli::parse();

    let path = args.fname;
    let mut store = ActionKV::open(&path).expect("unable to open file");
    store.load().expect("unable to load data");

    match &args.command {
        Some(command) => command.execute(&mut store, index_cache),
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
                            Ok(command) => command.execute(&mut store, index_cache),
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
