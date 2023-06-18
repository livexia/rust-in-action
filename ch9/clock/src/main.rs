use chrono::Local;
use clap::{Arg, Command};

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
    fn set(_format: &str, _datetime: &str) -> ! {
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
            Clock::set(format, datetime);
        }
        Some(_) | None => {
            let now = Clock::get(format);
            println!("{now}");
        }
    }
}
