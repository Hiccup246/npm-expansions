use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

use npm_expansions::npm_controller::NpmController;
use npm_expansions::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use npm_expansions::request::Request;
use npm_expansions::router;
use npm_expansions::routes_config;
use std::env;

fn main() {
    let development_env = env::var("DEV").is_ok();
    let addr = if development_env {
        "0.0.0.0:8080"
    } else {
        "[::]:8080"
    };

    let router = router::Router::new(routes_config::route_config());
    let listener = TcpListener::bind(addr).unwrap();

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
            HashMap::new(),
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
    use npm_expansions::mock_tcp_stream::MockTcpStream;

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

        fn valid_error_response(
            _error: &NpmExpansionsError,
            error_request: &Request,
        ) -> Result<Vec<u8>, NpmExpansionsError> {
            return NpmController::client_error(&error_request);
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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
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
                NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            handle_connection(&mut stream, &router, invalid_error_response);
        }
    }
}
