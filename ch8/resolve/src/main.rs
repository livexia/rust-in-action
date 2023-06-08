use std::error::Error;
use std::net::SocketAddr;

use clap::{arg, Command};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

// see: https://docs.rs/trust-dns-client/latest/trust_dns_client/
fn trust_dns_client_udp(domain_name: &str, dns_server: SocketAddr) -> Result<(), Box<dyn Error>> {
    let conn = UdpClientConnection::new(dns_server)?;
    let client = SyncClient::new(conn);
    let name: Name = domain_name.parse()?;
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A)?;

    let answers: &[Record] = response.answers();

    for answer in answers {
        if let Some(RData::A(ref ip)) = answer.data() {
            println!("{}", ip.to_string());
        }
    }
    Ok(())
}

fn trust_dns_client_msg_udp(
    domain_name: &str,
    dns_server: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("resolve")
        .version("0.1")
        .about("A simple to use DNS resolver")
        .arg(arg!(<"domain-name">).required(true))
        .arg(arg!(-s["dns-server"]).default_value("8.8.8.8"))
        .get_matches();

    let domain_name = matches.get_one::<String>("domain-name");
    let dns_server = matches.get_one::<String>("dns-server");
    match (domain_name, dns_server) {
        (Some(domain_name), Some(dns_server)) => {
            match format!("{dns_server}:53").parse::<SocketAddr>() {
                Ok(dns_server) => {
                    trust_dns_client_udp(domain_name, dns_server)?;
                    trust_dns_client_msg_udp(domain_name, dns_server)?;
                    Ok(())
                }
                Err(err) => {
                    eprintln!("Unable to parse dns-server");
                    Err(Box::new(err))
                }
            }
        }
        // unreachable!
        _ => Err(Box::new(clap::Error::new(
            clap::error::ErrorKind::DisplayHelp,
        ))),
    }
}
