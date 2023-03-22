use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Read, Write},
};

pub struct Request {
    status_line: String,
    headers: HashMap<String, String>,
}

impl Request {
    /// Builds a request object from a given http request stream
    ///
    /// # Arguments
    ///
    /// * `stream` - An incoming http request stream
    ///
    /// # Examples
    ///
    /// ```
    /// let input_bytes = b"GET / HTTP/1.1\r\n\r\n";
    /// let mut contents = vec![0u8; 1024];
    /// contents[..input_bytes.len()].clone_from_slice(input_bytes);
    /// let stream = MockTcpStream {
    ///     read_data: contents,
    ///     write_data: Vec::new(),
    /// };
    /// let request = Request::build(stream).unwrap();
    /// assert_eq!(request.status_line(), "GET / HTTP/1.1")
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the the given request stream is invalid. This can be due to
    /// the request having too many headers, having no status line, having invalid headers or
    /// the server being unable to process the stream.
    ///
    /// ```rust,should_error
    /// // fails if no http status line is given
    /// let input_bytes = b"";
    /// let mut contents = vec![0u8; 1024];
    /// contents[..input_bytes.len()].clone_from_slice(input_bytes);
    /// let stream = MockTcpStream {
    ///     read_data: contents,
    ///     write_data: Vec::new(),
    /// };
    /// Request::build(stream)
    /// ```
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

        // TODO Make this error more specific. This error could be too many headers or no blank line to mark end of headers.
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

        #[test]
        fn invalid_status_line() {
            let input_bytes = b"";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(stream);

            assert!(request.is_err())
        }

        #[test]
        fn invalid_headers() {
            let input_bytes = b"GET / HTTP/1.1\r\nContent-Type jndfjdnf\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(stream);

            assert!(request.is_err())
        }

        #[test]
        fn request_has_no_empty_line() {
            let input_bytes = b"GET / HTTP/1.1\r\n Content-Type: application/json";
            let mut contents = vec![0u8; 1024];
            let mut is_correct_error = false;

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(stream);

            if let Err(err) = request {
                is_correct_error = match err.kind() {
                    NpmErrorKind::RequestParseError => true,
                    _ => false,
                };
            }

            assert_eq!(is_correct_error, true)
        }
    }
}
