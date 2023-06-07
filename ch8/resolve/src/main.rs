use std::error::Error;
use std::net::SocketAddr;

use clap::{arg, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("resolve")
        .version("0.1")
        .about("A simple to use DNS resolver")
        .arg(arg!(<"domain-name">).required(true))
        .arg(arg!(-s["dns-server"]).default_value("8.8.8.8"))
        .get_matches();

    let name = matches.get_one::<String>("domain-name");
    let server = matches.get_one::<String>("dns-server");

    match (name, server) {
        (Some(name), Some(server)) => match format!("{server}:53").parse::<SocketAddr>() {
            Ok(server_addr) => {
                todo!()
            }
            Err(err) => panic!("Unable to parse dns-server: {err}"),
        },
        // unreachable!
        _ => Err(Box::new(clap::Error::new(
            clap::error::ErrorKind::DisplayHelp,
        ))),
    }
}
