use std::{error::Error, fmt::Display, str::FromStr};

use chrono::Local;
use clap::{Arg, ArgAction, Command};
use color_eyre::{eyre::Context, Report};
use tracing::{info, instrument};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug)]
enum ClockError {
    ChronoParse(chrono::ParseError),
    TimestampParse(std::num::ParseIntError),
    Libc(std::io::Error),
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

impl From<std::io::Error> for ClockError {
    fn from(value: std::io::Error) -> Self {
        Self::Libc(value)
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

    // see: https://linux.die.net/man/2/settimeofday
    #[cfg(unix)]
    #[instrument]
    fn set(format: &str, datetime: &str, dry_run: bool) -> Result<(), ClockError> {
        use std::mem::zeroed;

        use chrono::DateTime;
        use libc::{settimeofday, suseconds_t, time_t, timeval, timezone};

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

        // convert input datetime's timezone with local timezone to avoid confusing
        let dt = dt.with_timezone(&Local);
        info!("set time with: {:?}", dt);

        if dry_run {
            return Ok(());
        }

        // UNSAFE: init the timeval struct with zeroed
        let mut tv: timeval = unsafe { zeroed() };

        tv.tv_sec = dt.timestamp() as time_t;
        tv.tv_usec = dt.timestamp_subsec_micros() as suseconds_t;

        // UNSAFE: set the datetime but do not change the timezon
        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            let result = settimeofday(&tv as *const timeval, mock_tz);
            if result != 0 {
                return Err(std::io::Error::last_os_error().into());
            }
        }

        Ok(())
    }

    #[cfg(windows)]
    fn set(_format: &str, _datetime: &str, dry_run: bool) -> ! {
        unimplemented!("set datetime on windows is ye to be implemented")
    }
}

#[instrument]
fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();

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
        .subcommand(
            Command::new("get")
                .about("Set local time with corresponding format and local timezone"),
        )
        .subcommand(
            Command::new("set")
                .about("Get local time with corresponding format")
                .arg(Arg::new("datetime").required(true))
                .arg(
                    Arg::new("dry run")
                        .long("dry-run")
                        .short('d')
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    let format = matches.get_one::<String>("format").unwrap();

    match matches.subcommand() {
        Some(("set", set_matches)) => {
            let datetime = set_matches.get_one::<String>("datetime").unwrap();
            let dry_run = set_matches.get_flag("dry run");
            Clock::set(format, datetime, dry_run).wrap_err("unable to set the clock")?;
        }
        Some(_) | None => {
            let now = Clock::get(format);
            println!("{now}");
        }
    }
    Ok(())
}
