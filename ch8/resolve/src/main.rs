use std::error::Error;
use std::net::SocketAddr;

use clap::{arg, Command};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

// see: https://docs.rs/trust-dns-client/latest/trust_dns_client/
fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("resolve")
        .version("0.1")
        .about("A simple to use DNS resolver")
        .arg(arg!(<"domain-name">).required(true))
        .arg(arg!(-s["dns-server"]).default_value("8.8.8.8"))
        .get_matches();

    let name = matches.get_one::<String>("domain-name");
    let server = matches.get_one::<String>("dns-server");

    match (name, server) {
        (Some(name), Some(server)) => match format!("{server}:53").parse::<SocketAddr>() {
            Ok(server_addr) => {
                let conn = UdpClientConnection::new(server_addr)?;
                let client = SyncClient::new(conn);
                let name: Name = name.parse()?;
                let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A)?;

                let answers: &[Record] = response.answers();

                for answer in answers {
                    if let Some(RData::A(ref ip)) = answer.data() {
                        println!("{}", ip.to_string());
                    }
                }
                Ok(())
            }
            Err(err) => {
                eprintln!("Unable to parse dns-server");
                Err(Box::new(err))
            }
        },
        // unreachable!
        _ => Err(Box::new(clap::Error::new(
            clap::error::ErrorKind::DisplayHelp,
        ))),
    }
}
