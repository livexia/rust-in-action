use chrono::Local;
use clap::{Arg, Command};
use time::OffsetDateTime;

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
    dbg!(&format);

    match matches.subcommand() {
        Some(("set", set_matches)) => {
            dbg!(&set_matches);
        }
        Some(_) | None => {
            let chrono_local_now = Local::now();
            let time_local_now =
                OffsetDateTime::now_local().expect("time crate unable read time from local");
            println!("chrono crate local now: {}", chrono_local_now);
            println!("time crate local now: {:?}", time_local_now);
        }
    }
}
