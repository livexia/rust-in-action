use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{arg, Command};
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::{DnsResponse, Message, MessageType, OpCode, Query};
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::serialize::binary::{BinEncodable, BinEncoder};
use trust_dns_client::udp::UdpClientConnection;

// see: https://docs.rs/trust-dns-client/latest/trust_dns_client/
fn trust_dns_client_udp(
    domain_name: &str,
    dns_server: SocketAddr,
) -> Result<Vec<String>, Box<dyn Error>> {
    let conn = UdpClientConnection::new(dns_server)?;
    let client = SyncClient::new(conn);
    let domain_name: Name = domain_name.parse()?;
    let response: DnsResponse = client.query(&domain_name, DNSClass::IN, RecordType::A)?;

    let answers: &[Record] = response.answers();

    let mut records = vec![];
    for answer in answers {
        if let Some(RData::A(ref ip)) = answer.data() {
            records.push(ip.to_string());
        }
    }

    Ok(records)
}

fn trust_dns_client_msg_udp(
    domain_name: &str,
    dns_server: SocketAddr,
) -> Result<Vec<String>, Box<dyn Error>> {
    let domain_name: Name = domain_name.parse()?;
    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0")?;
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout))?;
    localhost.set_nonblocking(false)?;

    let _amt = localhost.send_to(&request_as_bytes, dns_server)?;

    let (_amt, _remote) = localhost.recv_from(&mut response_as_bytes)?;

    let dns_message = Message::from_vec(&response_as_bytes)?;

    let answers: &[Record] = dns_message.answers();

    let mut records = vec![];
    for answer in answers {
        if let Some(RData::A(ref ip)) = answer.data() {
            records.push(ip.to_string());
        }
    }

    Ok(records)
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
                    let r1 = trust_dns_client_udp(domain_name, dns_server)?;
                    let r2 = trust_dns_client_msg_udp(domain_name, dns_server)?;
                    assert_eq!(r1, r2);
                    for ip in r1 {
                        println!("{ip}");
                    }
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
