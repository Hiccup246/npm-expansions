use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader, Read, Write},
};

pub struct Request {
    status_line: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
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
    /// use npm_expansions::request::Request;
    /// use npm_expansions::mock_tcp_stream::MockTcpStream;
    /// let input_bytes = b"GET / HTTP/1.1\r\n\r\n";
    /// let mut contents = vec![0u8; 1024];
    /// contents[..input_bytes.len()].clone_from_slice(input_bytes);
    /// let mut stream = MockTcpStream {
    ///     read_data: contents,
    ///     write_data: Vec::new(),
    /// };
    /// let request = Request::build(&mut stream).unwrap();
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
    /// use npm_expansions::request::Request;
    /// use npm_expansions::mock_tcp_stream::MockTcpStream;
    /// let input_bytes = b"";
    /// let mut contents = vec![0u8; 1024];
    /// contents[..input_bytes.len()].clone_from_slice(input_bytes);
    /// let mut stream = MockTcpStream {
    ///     read_data: contents,
    ///     write_data: Vec::new(),
    /// };
    /// Request::build(&mut stream);
    /// ```
    pub fn build(stream: &mut (impl Read + Write)) -> Result<Request, NpmExpansionsError> {
        let buf_reader = BufReader::new(stream);
        let mut buffer = buf_reader.take(8000).lines();

        let status_line;

        if let Some(Ok(line)) = buffer.next() {
            status_line = line;
        } else {
            return Err(NpmExpansionsError::new(NpmErrorKind::RequestParseError));
        }

        let query_params = Self::build_query_params(&status_line)?;

        let mut headers: HashMap<String, String> = HashMap::new();

        for line in buffer {
            if let Ok(current_line) = line {
                if current_line.is_empty() {
                    return Ok(Request {
                        status_line,
                        headers,
                        query_params,
                    });
                } else {
                    let colon_position = current_line.find(':');

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

    fn build_query_params(
        status_line: &str,
    ) -> Result<HashMap<String, String>, NpmExpansionsError> {
        // GET /random?query=123&search=abc HTTP/1.1
        let split_line: Vec<&str> = status_line.split(' ').collect();

        if split_line.len() != 3 {
            return Err(NpmExpansionsError::new(NpmErrorKind::RequestParseError));
        }

        let uri = split_line
            .get(1)
            .ok_or(NpmExpansionsError::new(NpmErrorKind::RequestParseError));

        let query_params = uri?.split_once('?');

        if let Some(query_params) = query_params {
            let mut params = query_params.1;

            if params.is_empty() {
                return Ok(HashMap::new());
            }

            if let Some(param) = params.split_once('#') {
                params = param.0
            }

            let single: Vec<&str> = params.split('&').filter(|a| !a.is_empty()).collect();
            let vec_params: Result<Vec<(&str, &str)>, NpmExpansionsError> = single
                .iter()
                .map(|param| {
                    param
                        .split_once('=')
                        .ok_or(NpmExpansionsError::new(NpmErrorKind::RequestParseError))
                })
                .collect();

            let final_hashmap = vec_params?.iter().fold(HashMap::new(), |mut acc, param| {
                acc.insert(param.0.to_string(), param.1.to_string());
                acc
            });

            Ok(final_hashmap)
        } else {
            Ok(HashMap::new())
        }

        // Split into three parts
        // Get second part
        // Get ? onward
        // Split by & symbol
        // Split by = symbol
        // Put into Hashmap
    }

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

    pub fn status_line(&self) -> &str {
        self.status_line.as_str()
    }

    pub fn status_line_stripped(&self) -> String {
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

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

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
                &HashMap::from([("search".to_string(), "123".to_string())])
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
                    NpmErrorKind::RequestParseError => true,
                    _ => false,
                };
            }

            assert_eq!(is_correct_error, true)
        }
    }
}
