mod dns;
pub mod ethernet;

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
    let dns_server = matches.get_one::<String>("dns-server").unwrap();

    let url = Url::parse(url_str).expect("error: unable to parse <url> as a URL");
    let domain_name = url.host_str().expect("domain name required");

    if url.scheme() != "http" {
        unimplemented!("error: only HTTP protocol supported")
    }

    let remote_addr = dns::resolver(dns_server, domain_name).unwrap().unwrap();

    let mac = ethernet::MacAddress::new();

    dbg!(url);
    dbg!(remote_addr);
    dbg!(tun_iface_name);
    dbg!(mac);
}
