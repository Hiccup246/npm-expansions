use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Read, Write},
};

pub struct Request {
    status_line: String,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn build<'a>(mut stream: impl Read + Write) -> Request {
        let buf_reader = BufReader::new(&mut stream);
        let mut buffer = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty());

        let status_line = buffer.next().unwrap();

        let mut headers: HashMap<String, String> = HashMap::new();

        for header in buffer {
            let (key, value) = header.split_at(header.find(":").unwrap());
            let (_colon, header_value) = value.split_at(1);
            headers.insert(key.trim().to_string(), header_value.trim().to_string());
        }

        Request {
            status_line,
            headers,
        }
    }

    pub fn new<'a>(status_line: &str, headers: HashMap<String, String>) -> Request {
        Request {
            status_line: status_line.to_string(),
            headers: headers.clone(),
        }
    }

    pub fn status_line(&self) -> &str {
        self.status_line.as_str()
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_tcp_stream::MockTcpStream;

    #[test]
    fn parses_status_line() {
        let input_bytes = b"GET / HTTP/1.1\r\n\r\n";
        let mut contents = vec![0u8; 1024];

        contents[..input_bytes.len()].clone_from_slice(input_bytes);

        let stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };
        let request = Request::build(stream);

        assert_eq!(request.status_line(), "GET / HTTP/1.1")
    }

    #[test]
    fn parses_headers() {
        let input_bytes = b"GET / HTTP/1.1\n\rAccept: application/text,text/plain;q=0.1\r\nContent-Length: 0\r\n\r\n";
        let mut contents = vec![0u8; 1024];

        contents[..input_bytes.len()].clone_from_slice(input_bytes);

        let stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };
        let request = Request::build(stream);

        assert_eq!(
            request.headers(),
            &HashMap::from([
                ("Content-Length".to_string(), "0".to_string()),
                (
                    "Accept".to_string(),
                    "application/text,text/plain;q=0.1".to_string()
                ),
            ])
        )
    }
}
