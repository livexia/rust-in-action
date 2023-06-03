use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Command, FromArgMatches, Parser, Subcommand};
use libactionkv::ActionKV;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
}

impl Subcommands {
    fn execute(&self, store: &mut ActionKV) {
        match self {
            Subcommands::Get { key } => {
                println!("Get {key:?}: {:?}", store.get(key.as_bytes()));
            }
            Subcommands::Insert { key, value } => {
                println!("Insert {key:?} {value}");
                store.insert(key.as_bytes(), value.as_bytes()).unwrap();
            }
            Subcommands::Delete { key } => {
                println!("Delete {key:?}");
                store.delete(key.as_bytes()).unwrap();
            }
            Subcommands::Update { key, value } => {
                println!("Update {key:?} {value:?}");
                store.update(key.as_bytes(), value.as_bytes()).unwrap();
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    let path = args.fname;
    let mut store = ActionKV::open(&path).expect("unable to open file");
    store.load().expect("unable to load data");

    match &args.command {
        Some(command) => command.execute(&mut store),
        None => {
            let prompt = Command::new("Interactive prompt").no_binary_name(true);
            let mut prompt = Subcommands::augment_subcommands(prompt);
            loop {
                print!("[{path:?}]> ");
                io::stdout().flush().expect("unable to flush stdout");

                let mut buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("unable to readline from stdin");

                if let Some(raw_args) = shlex::split(&buffer) {
                    match prompt.try_get_matches_from_mut(raw_args.into_iter()) {
                        Ok(matches) => match Subcommands::from_arg_matches(&matches) {
                            Ok(command) => command.execute(&mut store),
                            Err(err) => eprintln!("{err}"),
                        },
                        Err(err) => eprintln!("{err}"),
                    }
                }
            }
        }
    }
}
