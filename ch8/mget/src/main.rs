// see: https://fasterthanli.me/articles/the-http-crash-course-nobody-asked-for#making-http-1-1-requests-ourselves

mod dns;
pub mod ethernet;
mod http11;

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    str::FromStr,
};

use clap::{arg, Command};
use color_eyre::eyre::eyre;
use nom::Offset;
use tracing::info;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};
// smoltcp::phy::TunTapInterface does not supported on mac
// use smoltcp::phy::TunTapInterface;
use url::Url;

fn main() -> color_eyre::Result<()> {
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
    let port = url.port().unwrap_or(80);
    let remote_addr = dns::resolver(dns_server, domain_name).unwrap().unwrap();
    let remote_addr: SocketAddr = format!("{remote_addr}:{port}")
        .parse()
        .expect("unable to parse SockAddr");

    if url.scheme() != "http" {
        unimplemented!("error: only HTTP protocol supported")
    }

    let mac = ethernet::MacAddress::new();

    dbg!(&url);
    dbg!(remote_addr);
    dbg!(tun_iface_name);
    dbg!(mac);

    let req = [
        "GET / HTTP/1.1",
        &format!("host: {}", url.host_str().unwrap()),
        "user-agent: cool-frog/1.0",
        "connection: close",
        "",
        "",
    ]
    .join("\r\n");
    let mut stream = TcpStream::connect(remote_addr).expect("unable to connect to remote addr");
    stream
        .write_all(req.as_bytes())
        .expect("unable to send request");

    let mut accum: Vec<u8> = Default::default();
    let mut rd_buf = [0u8; 1024];

    let (body_offset, res) = loop {
        let n = stream
            .read(&mut rd_buf)
            .expect("unable to read from stream");
        info!("Read {n} bytes");
        if n == 0 {
            return Err(eyre!(
                "unexpected EOF (server closed connection during headers)"
            ));
        }
        accum.extend_from_slice(&rd_buf[..n]);

        match http11::response(&accum) {
            Err(e) => {
                if e.is_incomplete() {
                    info!("Need to read more, continuing");
                    continue;
                } else {
                    return Err(eyre!("parse error: {e}"));
                }
            }
            Ok((remain, res)) => {
                // see returning from loops: https://doc.rust-lang.org/rust-by-example/flow_control/loop/return.html
                let body_offset = accum.offset(remain);
                break (body_offset, res);
            }
        }
    };

    info!("Got HTTP/1.1 response: {:#?}", res);
    let mut body_accum = accum[body_offset..].to_vec();

    let content_length = res
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .map(|(_, v)| v.parse::<usize>().unwrap())
        .unwrap_or_default();

    while body_accum.len() < content_length {
        let n = stream.read(&mut rd_buf[..])?;
        info!("Read {n} bytes");
        if n == 0 {
            return Err(eyre!(
                "unexpected EOF (server closed connection during headers)"
            ));
        }

        body_accum.extend_from_slice(&rd_buf[..n]);
    }

    info!("===== Response body =====");
    info!("{}", String::from_utf8_lossy(&body_accum));

    Ok(())
}
