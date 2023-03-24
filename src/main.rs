use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

mod accept_header_handler;
mod controller;
mod mime_type_parser;
mod mock_tcp_stream;
mod npm_expansion_error;
mod npm_expansions;
mod request;
mod response;
mod router;
mod routes_config;

pub use crate::npm_expansions::NpmExpansions;
pub use controller::Controller;
use npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
pub use request::Request;

fn main() {
    let router = router::Router::new(routes_config::route_config("static"));
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_connection(&mut stream, &router, http_response_error_handler);
    }
}

fn handle_connection(
    stream: &mut (impl Read + Write),
    router: &router::Router,
    error_handler: fn(&NpmExpansionsError, &Request) -> Result<Vec<u8>, NpmExpansionsError>,
) {
    let response = respond_to_request(stream, router);

    if let Err(res) = response {
        let error_request = Request::new(
            "",
            HashMap::from([(
                "Accept".to_string(),
                "text/html,application/json".to_string(),
            )]),
        );

        let error_response = error_handler(&res, &error_request);

        if let Ok(error_res) = error_response {
            return stream.write_all(error_res.as_slice()).unwrap();
        }

        panic!("Unconditional server failure. This is a server bug!")
    }
}

fn http_response_error_handler(
    error: &NpmExpansionsError,
    error_request: &Request,
) -> Result<Vec<u8>, NpmExpansionsError> {
    let error_response = match error.kind() {
        NpmErrorKind::InvalidHeader => Controller::client_error(error_request),
        NpmErrorKind::TooManyHeaders => Controller::client_error(error_request),
        NpmErrorKind::InvalidRequestStatusLine => Controller::client_error(error_request),
        NpmErrorKind::InternalServerError => Controller::internal_server_error(error_request),
        NpmErrorKind::RequestParseError => Controller::internal_server_error(error_request),
        NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(error_request),
    };

    error_response
}

fn respond_to_request(
    stream: &mut (impl Read + Write),
    router: &router::Router,
) -> Result<(), NpmExpansionsError> {
    let request = Request::build(stream)?;
    let response = router.route_request(request)?;

    stream.write_all(response.as_slice()).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;
    use crate::mock_tcp_stream::MockTcpStream;

    mod respond_to_request {
        use super::*;

        #[test]
        fn valid_http_stream() {
            let input_bytes = b"GET / HTTP/1.1\r\nAccept: text/html\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = respond_to_request(&mut stream, &router);

            assert!(response.is_ok());
        }

        #[test]
        fn no_not_found_route() {
            let input_bytes = b"GET /non-existant/route HTTP/1.1\r\nAccept: text/html\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = respond_to_request(&mut stream, &router);

            assert!(response.is_err());
        }

        #[test]
        fn invalid_request() {
            let input_bytes = b"";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = respond_to_request(&mut stream, &router);

            assert!(response.is_err());
        }
    }

    mod connection_handler {
        use super::*;

        fn valid_error_response(
            _error: &NpmExpansionsError,
            error_request: &Request,
        ) -> Result<Vec<u8>, NpmExpansionsError> {
            return Controller::client_error(&error_request);
        }

        fn invalid_error_response(
            _error: &NpmExpansionsError,
            _error_request: &Request,
        ) -> Result<Vec<u8>, NpmExpansionsError> {
            return Err(NpmExpansionsError::new(NpmErrorKind::InternalServerError));
        }

        #[test]
        fn valid_http_stream() {
            let input_bytes = b"GET / HTTP/1.1\r\nAccept: text/html\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = handle_connection(&mut stream, &router, valid_error_response);

            assert_eq!(response, ());
        }

        #[test]
        fn invalid_http_stream() {
            let input_bytes = b"";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = handle_connection(&mut stream, &router, valid_error_response);

            assert_eq!(response, ());
        }

        #[test]
        #[should_panic(expected = "Unconditional server failure. This is a server bug!")]
        fn invalid_http_stream_with_error_response() {
            let input_bytes = b"";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            handle_connection(&mut stream, &router, invalid_error_response);
        }
    }
}
