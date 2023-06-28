use chrono::{DateTime, Utc};

#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
    fn offset(&self) -> u32 {
        todo!()
    }

    fn delay(&self) -> u32 {
        todo!()
    }
}

fn ntp_roundtrip(server: &str, ntp_port: u16) -> Result<NTPResult, std::io::Error> {
    todo!()
}
fn weighted_mean(offsets: &[f64], offset_weights: &[f64]) -> f64 {
    todo!()
}

fn check_time() -> Result<f64, std::io::Error> {
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
