use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Lines, Read, Take, Write},
};

/// A struct representing the basic parts of a HTTP request i.e. status line, headers and query params
pub struct Request {
    status_line: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
}

const HEADER_SIZE_LIMIT: u64 = 8000;

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
    /// use npm_expansions::{
    ///     request::Request,
    ///     mock_tcp_stream::MockTcpStream,
    /// };
    ///
    /// let input_bytes = b"GET / HTTP/1.1\r\n\r\n";
    /// let mut contents = vec![0u8; 1024];
    /// contents[..input_bytes.len()].clone_from_slice(input_bytes);
    /// let mut stream = MockTcpStream {
    ///     read_data: contents,
    ///     write_data: Vec::new(),
    /// };
    /// let request = Request::build(&mut stream).unwrap();
    ///
    /// assert_eq!(request.status_line(), "GET / HTTP/1.1");
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
    /// use npm_expansions::{
    ///     request::Request,
    ///     mock_tcp_stream::MockTcpStream,
    /// };
    ///
    /// let input_bytes = b"";
    /// let mut contents = vec![0u8; 1024];
    /// contents[..input_bytes.len()].clone_from_slice(input_bytes);
    /// let mut stream = MockTcpStream {
    ///     read_data: contents,
    ///     write_data: Vec::new(),
    /// };
    ///
    /// Request::build(&mut stream);
    /// ```
    pub fn build(stream: &mut (impl Read + Write)) -> Result<Request, NpmExpansionsError> {
        let buf_reader = BufReader::new(stream);
        let mut buffer = buf_reader.take(HEADER_SIZE_LIMIT).lines();

        let status_line;

        if let Some(Ok(line)) = buffer.next() {
            status_line = line;
        } else {
            return Err(NpmExpansionsError::from(NpmErrorKind::InvalidHttpRequest));
        }

        let query_params = Self::build_query_params(&status_line)?;
        let headers = Self::build_headers(&mut buffer)?;

        Ok(Request {
            status_line,
            headers,
            query_params,
        })
    }

    fn build_headers(
        header_buffer: &mut Lines<Take<BufReader<&mut (impl Read + Write)>>>,
    ) -> Result<HashMap<String, String>, NpmExpansionsError> {
        let mut headers: HashMap<String, String> = HashMap::new();

        for line in header_buffer {
            let current_line =
                line.map_err(|_| NpmExpansionsError::from(NpmErrorKind::InvalidHttpRequest))?;

            if current_line.is_empty() {
                return Ok(headers);
            } else {
                let (key, value) = Self::header_key_value(current_line)?;
                headers.insert(key, value);
            }
        }

        // TODO Make this error more specific. This error could be too many headers or no blank line to mark end of headers.
        Err(NpmExpansionsError::from(NpmErrorKind::InvalidHttpRequest))
    }

    fn header_key_value(header_line: String) -> Result<(String, String), NpmExpansionsError> {
        let colon_position = header_line.find(':').ok_or(NpmExpansionsError::from(
            NpmErrorKind::InvalidRequestHeaders,
        ))?;

        let (key, value) = header_line.split_at(colon_position);
        let (_colon, header_value) = value.split_at(1);

        Ok((key.trim().to_string(), header_value.trim().to_string()))
    }

    fn build_query_params(
        status_line: &str,
    ) -> Result<HashMap<String, String>, NpmExpansionsError> {
        let split_line: Vec<&str> = status_line.split(' ').collect();

        let uri = split_line
            .get(1)
            .ok_or(NpmExpansionsError::from(NpmErrorKind::InvalidHttpRequest));

        let query_params = uri?.split_once('?').unwrap_or(("", ""));

        if query_params.1.is_empty() {
            return Ok(HashMap::new());
        }

        Self::process_query_string(query_params.1)
    }

    fn process_query_string(
        query_string: &str,
    ) -> Result<HashMap<String, String>, NpmExpansionsError> {
        let query_parameters: Result<Vec<(&str, &str)>, NpmExpansionsError> = query_string
            .split('&')
            .filter(|a| !a.is_empty())
            .map(|param| {
                param
                    .split_once('=')
                    .ok_or(NpmExpansionsError::from(NpmErrorKind::InvalidHttpRequest))
            })
            .collect();

        let query_hash_map: HashMap<String, String> =
            query_parameters?
                .into_iter()
                .fold(HashMap::new(), |mut acc, param| {
                    let (key, value) = param;
                    acc.insert(key.to_string(), value.to_string());
                    acc
                });

        Ok(query_hash_map)
    }

    /// Creates a new request from status line string, headers HashMap and query params hashmap
    pub fn new(
        status_line: &str,
        headers: HashMap<String, String>,
        query_params: HashMap<String, String>,
    ) -> Request {
        Request {
            status_line: status_line.to_string(),
            headers,
            query_params,
        }
    }

    /// Returns the status line of a request object
    pub fn status_line(&self) -> &str {
        self.status_line.as_str()
    }

    /// Returns the path from the status line of a request object
    pub fn status_line_path(&self) -> String {
        let split_line: Vec<&str> = self.status_line.split(' ').collect();

        let method = *split_line.first().unwrap_or(&"");
        let mut route = *split_line.get(1).unwrap_or(&"");
        let version = *split_line.get(2).unwrap_or(&"");

        route = if let Some(split) = route.split_once('?') {
            split.0
        } else {
            route
        };

        format!("{method} {route} {version}")
    }

    /// Returns the requests headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Returns the requests query params
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
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

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(request.status_line(), "GET / HTTP/1.1")
        }

        #[test]
        fn parses_query_params() {
            let input_bytes = b"GET /random?query=123&search=hello HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(
                request.query_params(),
                &HashMap::from([
                    ("query".to_string(), "123".to_string()),
                    ("search".to_string(), "hello".to_string())
                ])
            )
        }

        #[test]
        fn no_query_params() {
            let input_bytes = b"GET /random HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(request.query_params(), &HashMap::new())
        }

        #[test]
        fn empty_query_params() {
            let input_bytes = b"GET /random? HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(request.query_params(), &HashMap::new())
        }

        #[test]
        fn single_query_param() {
            let input_bytes = b"GET /random?search=123 HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(
                request.query_params(),
                &HashMap::from([("search".to_string(), "123".to_string())])
            )
        }

        #[test]
        fn malformed_single_query_param() {
            let input_bytes = b"GET /random?search HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream);

            assert!(request.is_err())
        }

        #[test]
        fn malformed_two_single_query_param() {
            let input_bytes = b"GET /random?search& HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream);

            assert!(request.is_err())
        }

        #[test]
        fn multi_malformed_query_param() {
            let input_bytes = b"GET /random?search=123& HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(
                request.query_params(),
                &HashMap::from([("search".to_string(), "123".to_string())])
            )
        }

        #[test]
        fn blank_query_param() {
            let input_bytes = b"GET /random?search= HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(
                request.query_params(),
                &HashMap::from([("search".to_string(), "".to_string())])
            )
        }

        #[test]
        fn malformedmulti_query_param() {
            let input_bytes = b"GET /random?&&&&& HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(request.query_params(), &HashMap::new())
        }

        #[test]
        fn includes_hashtag() {
            let input_bytes = b"GET /random?search=123#test HTTP/1.1\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

            assert_eq!(
                request.query_params(),
                &HashMap::from([("search".to_string(), "123#test".to_string())])
            )
        }

        #[test]
        fn parses_headers() {
            let input_bytes = b"GET / HTTP/1.1\n\rAccept: application/text,text/plain;q=0.1\r\nContent-Length: 0\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream).unwrap();

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

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream);

            assert!(request.is_err())
        }

        #[test]
        fn invalid_headers() {
            let input_bytes = b"GET / HTTP/1.1\r\nContent-Type jndfjdnf\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream);

            assert!(request.is_err())
        }

        #[test]
        fn request_has_no_empty_line() {
            let input_bytes = b"GET / HTTP/1.1\r\n Content-Type: application/json";
            let mut contents = vec![0u8; 1024];
            let mut is_correct_error = false;

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };
            let request = Request::build(&mut stream);

            if let Err(err) = request {
                is_correct_error = match err.kind() {
                    NpmErrorKind::InvalidHttpRequest => true,
                    _ => false,
                };
            }

            assert_eq!(is_correct_error, true)
        }
    }
}
