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
    fn execute(&self, store: &mut ActionKV, index_cache: bool) {
        let mut modified = true;
        if index_cache && modified {
            if let Err(err) = read_cache_from_disk(store) {
                eprintln!("{err}");
            }
        }
        match self {
            Subcommands::Get { key } | Subcommands::Show { key } => match store.get(key.as_bytes())
            {
                Ok(value) => {
                    modified = false;
                    match value {
                        None => println!("None"),
                        Some(value) => {
                            if let Subcommands::Get { .. } = self {
                                println!("{:?}: {:?}", key.as_bytes(), value)
                            } else {
                                println!("{key:?}: {:?}", String::from_utf8_lossy(&value))
                            }
                        }
                    }
                }
                Err(err) => eprintln!("{err:?}"),
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
        if index_cache && modified {
            if let Err(err) = write_index_to_disk(store) {
                eprintln!("{err}");
            }
        }
    }
}

fn read_cache_from_disk(store: &mut ActionKV) -> Result<(), std::io::Error> {
    match store.get(INDEX_KEY.as_bytes()) {
        Ok(value) => {
            if let Some(index_as_bytes) = value {
                match bincode::deserialize::<Cache>(&index_as_bytes) {
                    Ok(index) => store.index = index,
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

fn write_index_to_disk(store: &mut ActionKV) -> Result<(), std::io::Error> {
    match bincode::serialize(&store.index) {
        Ok(index_as_bytes) => match store.insert(INDEX_KEY.as_bytes(), &index_as_bytes) {
            Ok(_) => Ok(println!("Insert index cache")),
            Err(err) => Err(err),
        },
        Err(err) => Err(Error::new(ErrorKind::Other, err)),
    }
}

pub fn run(index_cache: bool) {
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
