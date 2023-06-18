use std::{error::Error, fmt::Display};

use chrono::Local;
use clap::{Arg, Command};

#[derive(Debug)]
enum ClockError {
    ChronoParse(chrono::ParseError),
    TimestampParse(std::num::ParseIntError),
}

impl Display for ClockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ClockError {}

impl From<chrono::ParseError> for ClockError {
    fn from(value: chrono::ParseError) -> Self {
        Self::ChronoParse(value)
    }
}

impl From<std::num::ParseIntError> for ClockError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::TimestampParse(value)
    }
}

struct Clock;

impl Clock {
    fn get(format: &str) -> String {
        match format {
            "timestamp" => Local::now().timestamp().to_string(),
            "rfc2822" => Local::now().to_rfc2822(),
            "rfc3339" => Local::now().to_rfc3339(),
            _ => unreachable!(),
        }
    }

    #[cfg(unix)]
    fn set(format: &str, datetime: &str) -> Result<(), ClockError> {
        use chrono::DateTime;

        let dt = match format {
            "timestamp" => {
                let unix_ts: i64 = datetime.parse()?;
                dbg!(unix_ts);
                todo!()
            }
            "rfc2822" => DateTime::parse_from_rfc2822(datetime),
            "rfc3339" => DateTime::parse_from_rfc3339(datetime),
            _ => unreachable!(),
        }?;

        println!("{:?}", dt);
        unimplemented!("set datetime in unix is yet to be implemented")
    }

    #[cfg(windows)]
    fn set(_format: &str, _datetime: &str) -> ! {
        unimplemented!("set datetime on windows is ye to be implemented")
    }
}

fn main() {
    let matches = Command::new("Clock")
        .version("0.1")
        .about("Gets and (aspirationally) sets the time")
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .value_parser(["timestamp", "rfc2822", "rfc3339"])
                .default_value("rfc3339"),
        )
        .subcommand(Command::new("get").about("Set local time with corresponding format"))
        .subcommand(
            Command::new("set")
                .about("Get local time with corresponding format")
                .arg(Arg::new("datetime").required(true)),
        )
        .get_matches();

    let format = matches.get_one::<String>("format").unwrap();

    match matches.subcommand() {
        Some(("set", set_matches)) => {
            let datetime = set_matches.get_one::<String>("datetime").unwrap();
            Clock::set(format, datetime).expect("unable to set the clock");
        }
        Some(_) | None => {
            let now = Clock::get(format);
            println!("{now}");
        }
    }
}
