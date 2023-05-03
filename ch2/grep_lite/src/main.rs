use clap::{arg, Command};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn search_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for _line in reader.lines() {
        let line = _line.unwrap();
        let contains_substring = re.find(&line);
        if contains_substring.is_some() {
            println!("{}", line)
        }
    }
}

fn main() {
    let matches = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(arg!(<pattern> "The pattern to search for").required(true))
        .arg(arg!(<input> "File to search").required(false))
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").expect("required");
    let re = Regex::new(pattern).unwrap();

    let input = matches.get_one::<String>("input");
    match input {
        Some(input) => {
            let f = File::open(input).unwrap();
            let reader = BufReader::new(f);
            search_lines(reader, re)
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            search_lines(reader, re)
        }
    }
}
