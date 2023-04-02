use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::npm_controller::NpmController;
use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use crate::request::Request;
use crate::router;

pub fn handle_connection(stream: &mut (impl Read + Write), router: &router::Router) {
    let response = respond_to_request(stream, router);

    if let Err(res) = response {
        let error_request = Request::new(
            "",
            HashMap::from([(
                "Accept".to_string(),
                "text/html,application/json".to_string(),
            )]),
            HashMap::new(),
        );

        let error_response = http_response_error_handler(&res, &error_request);

        if let Ok(error_res) = error_response {
            return stream.write_all(error_res.as_slice()).unwrap();
        }

        println!("Unconditional server failure. This is a server bug!")
    }
}

fn http_response_error_handler(
    error: &NpmExpansionsError,
    error_request: &Request,
) -> Result<Vec<u8>, NpmExpansionsError> {
    let error_response = match error.kind() {
        NpmErrorKind::InvalidHeader => NpmController::client_error(error_request),
        NpmErrorKind::TooManyHeaders => NpmController::client_error(error_request),
        NpmErrorKind::InvalidRequestStatusLine => NpmController::client_error(error_request),
        NpmErrorKind::InternalServerError => NpmController::internal_server_error(error_request),
        NpmErrorKind::RequestParseError => NpmController::internal_server_error(error_request),
        NpmErrorKind::SupportedMimeTypeError => NpmController::internal_server_error(error_request),
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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = respond_to_request(&mut stream, &router);

            assert!(response.is_err());
        }
    }

    mod connection_handler {
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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = handle_connection(&mut stream, &router);

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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = handle_connection(&mut stream, &router);

            assert_eq!(response, ());
        }
    }
}
