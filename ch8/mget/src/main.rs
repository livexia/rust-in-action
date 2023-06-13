// see: https://fasterthanli.me/articles/the-http-crash-course-nobody-asked-for#making-http-1-1-requests-ourselves

mod dns;
pub mod ethernet;

use std::str::FromStr;

use clap::{arg, Command};
use tracing::info;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};
// smoltcp::phy::TunTapInterface does not supported on mac
// use smoltcp::phy::TunTapInterface;
use url::Url;

fn main() {
    color_eyre::install().unwrap();

    let filter_layer =
        Targets::from_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")).unwrap();
    let format_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();

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

    info!("url: {url_str}");
    info!("iface: {tun_iface_name}");
    info!("dns-server: {dns_server}");

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
