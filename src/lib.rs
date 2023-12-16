mod method;
mod response;
mod router;
mod utils;

use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

use httparse::EMPTY_HEADER;
use smallvec::SmallVec;

use self::utils::memmem;
pub use self::{
    method::Method,
    response::{IntoResponse, Response},
    router::Router,
};

pub const REQUEST_DELIMITER: &[u8] = b"\r\n\r\n";
pub const LINE_DELIMITER: &[u8] = b"\r\n";
const MAX_HEADERS: usize = 256;

#[derive(Debug, Default)]
pub struct Server<'a> {
    router: Router<'a>,
}

impl<'a> Server<'a> {
    pub fn new(router: Router<'a>) -> Self {
        Self { router }
    }

    pub fn serve(mut self) {
        let address = (Ipv4Addr::LOCALHOST, 8080);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_request(stream);
        }
    }

    fn handle_request(&mut self, mut stream: TcpStream) {
        let request = read_and_parse_request(&mut stream);

        let response = {
            let Some(handler) = self.router.lookup_handler(request.method, &request.path) else {
                panic!("unknown method+path");
            };

            handler(request)
        };
        self.write_response(stream, response);
    }

    fn write_response(&self, mut stream: TcpStream, response: Response) {
        let Response {
            body,
            status_code,
            headers,
        } = response;

        // send response and flush it
        write!(stream, "HTTP/1.1 {status_code} OK\n\r").unwrap();
        for (key, value) in headers {
            write!(stream, "{key}: {value}\n\r").unwrap();
        }

        let body = body.as_deref().unwrap_or("");
        write!(stream, "\n\r{body}").unwrap();

        stream.flush().unwrap();
    }
}

fn read_and_parse_request(stream: &mut TcpStream) -> Request {
    let mut buf = Vec::new();
    let mut tmp_buf = [0; 2048];

    let delimiter_position = loop {
        let bytes_read = stream.read(&mut tmp_buf).unwrap();

        let buf_len = buf.len();
        let delimiter_check_start = buf_len.saturating_sub(4);

        if bytes_read == 0 {
            panic!("unexpected eof");
        }

        buf.extend(&tmp_buf[..bytes_read]);

        if let Some(position) = memmem(&buf[delimiter_check_start..], b"\r\n\r\n") {
            break position;
        }
    };
    let body_start = delimiter_position + 4;

    let (request_head, _body) = buf.split_at(body_start);

    let mut header_array = [EMPTY_HEADER; MAX_HEADERS];
    let mut request = httparse::Request::new(&mut header_array);

    httparse::Request::parse(&mut request, request_head)
        .expect("Failed to parse request")
        .unwrap();

    let mut content_length = None;

    let headers = request
        .headers
        .iter()
        .take_while(|&&header| header != EMPTY_HEADER)
        .map(|header| {
            if header.name == "Content-Length" {
                content_length = Some(header.value);
            }

            let value = String::from_utf8(header.value.to_vec()).unwrap();
            (header.name.into(), value)
        })
        .collect();

    let request = Request {
        method: request.method.unwrap().parse().unwrap(),
        path: request.path.unwrap().to_string(),
        version: request.version.unwrap().to_string(),
        headers,
        body: String::new(),
    };

    let body = if let Some(content_length) = content_length {
        let content_length = std::str::from_utf8(content_length).unwrap().parse::<usize>().unwrap();
        while buf.len() - body_start != content_length {
            let bytes_read = stream.read(&mut tmp_buf).unwrap();

            if bytes_read == 0 {
                panic!("unexpected eof");
            }

            buf.extend(&tmp_buf[..bytes_read]);
        }

        std::str::from_utf8(&buf[body_start..]).unwrap().to_string()
    } else {
        String::new()
    };

    Request { body, ..request }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: HeaderMap,
    pub body: String,
}

type HeaderMap = SmallVec<[(String, String); 64]>;
