#[derive(Debug)]
pub struct Response<'a> {
    pub status: u16,
    pub status_text: &'a str,

    // header name/values could be non-UTF8, but ignore for the example
    // do not use HashMap, headers can repeat
    pub headers: Vec<(&'a str, &'a str)>,
}

const CRLF: &str = "\r\n";

// Loos like `HTTP/1.1 200 OK\r\n` or `HTTP/1.1 404 Not Found\r\n`
pub fn response(i: &[u8]) -> Result<(&[u8], Response<'_>), Box<dyn std::error::Error>> {
    todo!()
}
