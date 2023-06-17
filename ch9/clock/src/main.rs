use chrono::Local;
use time::OffsetDateTime;

fn main() {
    let chrono_local_now = Local::now();
    let time_local_now =
        OffsetDateTime::now_local().expect("time crate unable read time from local");
    println!("chrono crate local now: {}", chrono_local_now);
    println!("time crate local now: {:?}", time_local_now);
}
