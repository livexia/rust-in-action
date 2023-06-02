use std::path::PathBuf;

use clap::{Parser, Subcommand};
use libactionkv::ActionKV;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// FILE for ActionKV
    #[arg(value_name = "FILE")]
    fname: PathBuf,

    /// Operation commands
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Retrieves the value at key from the store
    Get { key: String },
    /// Adds a key-value pair to the store
    Insert { key: String, value: String },
    /// Removes a key-value pair from the store
    Delete { key: String },
    /// Replaces an old value with a new one
    Update { key: String, value: String },
}

fn main() {
    let args = Cli::parse();

    let path = args.fname;
    let mut store = ActionKV::open(&path).expect("unable to open file");
    store.load().expect("unable to load data");

    match &args.command {
        Commands::Get { key } => {
            println!("Get {key}: {:?}", store.get(key.as_bytes()));
        }
        Commands::Insert { key, value } => {
            println!("Insert {key} {value}");
            store.insert(key.as_bytes(), value.as_bytes()).unwrap();
        }
        Commands::Delete { key } => {
            println!("Delete {key}");
            store.delete(key.as_bytes()).unwrap();
        }
        Commands::Update { key, value } => {
            println!("Update {key} {value}");
            store.update(key.as_bytes(), value.as_bytes()).unwrap();
        }
    }
}
