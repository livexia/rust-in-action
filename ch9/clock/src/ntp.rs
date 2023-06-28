use chrono::{DateTime, Utc};

#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
    fn offset(&self) -> i64 {
        (((self.t2 - self.t1) + (self.t4 - self.t3)) / 2).num_milliseconds()
    }

    fn delay(&self) -> i64 {
        ((self.t4 - self.t1) - (self.t3 - self.t2)).num_milliseconds()
    }
}

fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult, std::io::Error> {
    let destination = format!("{}:{}", host, port);
    println!("ntp server: {destination}");
    todo!()
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
        print!("{} =>", server);

        let calc = ntp_roundtrip(server, NTP_PORT);

        match calc {
            Ok(time) => {
                println!(" {}ms away from local system time", time.offset());
                times.push(time);
            }
            Err(_) => {
                println!(" ? [response took too long]");
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
