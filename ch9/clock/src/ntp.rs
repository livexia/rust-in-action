use std::{net::UdpSocket, time::Duration};

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Utc};
use tracing::info;

// without authenticator
const NTP_MESSAGE_LENGTH: usize = 48;
const LOCAL_ADDR: &str = "0.0.0.0:1230";

// see: https://stackoverflow.com/a/29138806
const NTP_TO_UNIX_SECONDS: i64 = (70 * 365 + 17) * 86400;

#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

#[derive(Debug)]
struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

#[derive(Debug)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

impl NTPResult {
    // wrong offset defintion in the book
    // see: https://github.com/rust-in-action/code/issues/86
    fn offset(&self) -> i64 {
        ((self.t2 - self.t1) + (self.t3 - self.t4)).num_milliseconds() / 2
    }

    fn delay(&self) -> i64 {
        ((self.t4 - self.t1) - (self.t3 - self.t2)).num_milliseconds()
    }
}

impl NTPMessage {
    fn new() -> Self {
        Self {
            data: [0; NTP_MESSAGE_LENGTH],
        }
    }

    fn client() -> Self {
        const VN: u8 = 0b00_100_000; // version 4
        const MODE: u8 = 0b011; // mode client

        let mut msg = Self::new();
        msg.data[0] |= VN;
        msg.data[0] |= MODE;

        msg
    }

    fn parse_timestamp(&self, i: usize) -> Result<NTPTimestamp, std::io::Error> {
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimestamp { seconds, fraction })
    }

    fn rx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(32)
    }

    fn tx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(40)
    }
}

impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nsecs = ntp.fraction as f64;

        nsecs *= 1e9;
        nsecs /= 2_f64.powi(32);

        Utc.timestamp_opt(secs, nsecs as u32).unwrap()
    }
}

fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult, std::io::Error> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);

    let request = NTPMessage::client();
    let mut response = NTPMessage::new();

    let message = request.data;

    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    udp.connect(destination).expect("unable to connect");

    let t1 = Utc::now();

    udp.send(&message)?;
    udp.set_read_timeout(Some(timeout))?;
    udp.recv_from(&mut response.data)?;

    let t4 = Utc::now();

    let t2 = response.rx_time()?.into();

    let t3 = response.tx_time()?.into();

    Ok(NTPResult { t1, t2, t3, t4 })
}

// see: https://en.wikipedia.org/wiki/Weighted_arithmetic_mean
fn weighted_mean(offsets: &[f64], offset_weights: &[f64]) -> f64 {
    offsets
        .iter()
        .zip(offset_weights)
        .fold(0f64, |s, (x, w)| s + x * w)
        / offset_weights.iter().sum::<f64>()
}

pub fn check_time() -> Result<f64, std::io::Error> {
    const NTP_PORT: u16 = 123;

    let servers = [
        "time.nist.gov",
        "ntp1.aliyun.com",
        // "time.windows.com",
        "time.apple.com",
        "time.google.com",
        "cn.ntp.org.cn",
        "ntp.ntsc.ac.cn",
        "ntp.tuna.tsinghua.edu.cn",
    ];

    let mut times = Vec::with_capacity(servers.len());

    for &server in &servers {
        let calc = ntp_roundtrip(server, NTP_PORT);

        match calc {
            Ok(time) => {
                info!(
                    "{} => {}ms away from local system time",
                    server,
                    time.offset()
                );
                times.push(time);
            }
            Err(_) => {
                info!("{} => ? [response took too long]", server);
            }
        }
    }

    let mut offsets = Vec::with_capacity(servers.len());
    let mut offset_weights = Vec::with_capacity(servers.len());

    for time in times {
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;

        let weight = 1_000_000.0 / (delay * delay);
        if weight.is_finite() {
            offsets.push(offset);
            offset_weights.push(weight);
        }
    }

    let avg_offset = weighted_mean(&offsets, &offset_weights);

    Ok(avg_offset)
}
