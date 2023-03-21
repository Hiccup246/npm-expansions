use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Read, Write},
};

use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};

pub struct Request {
    status_line: String,
    headers: HashMap<String, String>,
}

// impl fmt::Debug for dyn RequestParsingError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Request Parsing Error")
//     }
// }

impl Request {
    // Build should return a result which the main function will use to return as 500 if an error occured
    // Limit headers + status line to 8000 bytes
    pub fn build(mut stream: impl Read + Write) -> Result<Request, NpmExpansionsError> {
        let buf_reader = BufReader::new(&mut stream);
        let mut buffer = buf_reader.take(8000).lines();

        let status_line;

        if let Some(line) = buffer.next() {
            if let Ok(current_line) = line {
                status_line = current_line;
            } else {
                return Err(NpmExpansionsError::new(NpmErrorKind::RequestParseError));
            }
        } else {
            return Err(NpmExpansionsError::new(NpmErrorKind::RequestParseError));
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
                    let colon_position = current_line.find(":");

                    if let Some(colon_position) = colon_position {
                        let (key, value) = current_line.split_at(colon_position);
                        let (_colon, header_value) = value.split_at(1);

                        headers.insert(key.trim().to_string(), header_value.trim().to_string());
                    } else {
                        return Err(NpmExpansionsError::new(NpmErrorKind::InvalidHeader));
                    }
                }
            } else {
                return Err(NpmExpansionsError::new(NpmErrorKind::RequestParseError));
            }
        }

        Err(NpmExpansionsError::new(NpmErrorKind::RequestParseError))
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
