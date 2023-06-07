use clap::{arg, Command};

fn main() {
    let matches = Command::new("resolve")
        .version("0.1")
        .about("A simple to use DNS resolver")
        .arg(arg!(<"domain-name">).required(true))
        .arg(arg!(-s["dns-server"]).default_value("8.8.8.8"))
        // .arg(Arg::new("dns-server").short('s').default_value("8.8.8.8"))
        .get_matches();
    println!("{:?}", matches);
    println!("Hello, world!");
}
