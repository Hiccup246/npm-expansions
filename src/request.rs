use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Read, Write},
};

pub struct Request {
    status_line: String,
    headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RequestParseError;

impl Request {
    // Build should return a result which the main function will use to return as 500 if an error occured
    pub fn build(mut stream: impl Read + Write) -> Result<Request, RequestParseError> {
        let buf_reader = BufReader::new(&mut stream);
        let mut buffer = buf_reader.lines();

        let status_line;

        if let Some(line) = buffer.next() {
            if let Ok(current_line) = line {
                status_line = current_line;
            } else {
                return Err(RequestParseError);
            }
        } else {
            return Err(RequestParseError);
        }

        let mut headers: HashMap<String, String> = HashMap::new();

        for line in buffer {
            if let Ok(current_line) = line {
                if current_line.is_empty() {
                    return Ok(Request {
                        status_line,
                        headers,
                    });
                } else {
                    let (key, value) =
                        current_line.split_at(current_line.find(":").ok_or(RequestParseError)?);
                    let (_colon, header_value) = value.split_at(1);
                    headers.insert(key.trim().to_string(), header_value.trim().to_string());
                }
            } else {
                return Err(RequestParseError);
            }
        }
        Ok(Request {
            status_line,
            headers,
        })
    }

    pub fn new(status_line: &str, headers: HashMap<String, String>) -> Request {
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

    mod build_tests {
        use super::*;

        #[test]
        fn parses_status_line() {
            let input_bytes = b"GET / HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(stream).unwrap();

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
            let request = Request::build(stream).unwrap();

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
}
