mod dns;
mod ethernet;

use clap::{arg, Command};
// smoltcp::phy::TunTapInterface does not supported on mac
// use smoltcp::phy::TunTapInterface;
use url::Url;

fn main() {
    let matches = Command::new("mget")
        .version("0.1")
        .about("GET a webpage, manually")
        .arg(arg!(<"url">).required(true))
        .arg(arg!(<"tun-iface">).required(true))
        .arg(arg!(-s["dns-server"]).default_value("8.8.8.8"))
        .get_matches();

    let url_str = matches.get_one::<String>("url").unwrap();
    let tun_iface_name = matches.get_one::<String>("tun-iface").unwrap();

    let url = Url::parse(url_str).expect("error: unable to parse <url> as a URL");

    if url.scheme() != "http" {
        unimplemented!("error: only HTTP protocol supported")
    }
    println!("{:?}", url);
    println!("{:?}", tun_iface_name);
}
