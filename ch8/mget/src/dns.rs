use std::error::Error;
use std::fmt::Display;
use std::net::{AddrParseError, IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;

use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::error::ClientError;
use trust_dns_client::op::{DnsResponse, Message, MessageType, OpCode, Query};
use trust_dns_client::proto::error::ProtoError;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::serialize::binary::{BinEncodable, BinEncoder};
use trust_dns_client::udp::UdpClientConnection;

#[derive(Debug)]
pub enum ResolveError {
    DNSClient(ClientError),
    NameParse(ProtoError),
    AddrParse(AddrParseError),
    IO(std::io::Error),
    Clap(clap::Error),
}

impl Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ResolveError {}

impl From<ClientError> for ResolveError {
    fn from(value: ClientError) -> Self {
        Self::DNSClient(value)
    }
}

impl From<ProtoError> for ResolveError {
    fn from(value: ProtoError) -> Self {
        Self::NameParse(value)
    }
}

impl From<AddrParseError> for ResolveError {
    fn from(value: AddrParseError) -> Self {
        Self::AddrParse(value)
    }
}

impl From<std::io::Error> for ResolveError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<clap::Error> for ResolveError {
    fn from(value: clap::Error) -> Self {
        Self::Clap(value)
    }
}

// see: https://docs.rs/trust-dns-client/latest/trust_dns_client/
#[allow(dead_code)]
fn trust_dns_client_udp(
    domain_name: &str,
    dns_server: SocketAddr,
) -> Result<Vec<String>, ResolveError> {
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
    dns_server: SocketAddr,
    domain_name: &str,
) -> Result<Option<IpAddr>, ResolveError> {
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

    for answer in answers {
        if let Some(RData::A(ip)) = answer.data() {
            return Ok(Some(IpAddr::V4(*ip)));
        }
    }

    Ok(None)
}

pub fn resolver(dns_server_addr: &str, domain_name: &str) -> Result<Option<IpAddr>, ResolveError> {
    let dns_server = format!("{dns_server_addr}:53").parse()?;
    trust_dns_client_msg_udp(dns_server, domain_name)
}
