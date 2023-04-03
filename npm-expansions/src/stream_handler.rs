use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use crate::request::Request;
use crate::router;
use crate::{default_controller::DefaultController, expansions_model::ExpansionsAccess};

pub fn handle_connection(
    stream: &mut (impl Read + Write),
    router: &router::Router,
    expansions_generator: &dyn ExpansionsAccess,
) {
    let response = respond_to_request(stream, router, expansions_generator);

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
        NpmErrorKind::InvalidHeader => DefaultController::client_error(error_request),
        NpmErrorKind::TooManyHeaders => DefaultController::client_error(error_request),
        NpmErrorKind::InternalServerError => {
            DefaultController::internal_server_error(error_request)
        }
        NpmErrorKind::RequestParseError => DefaultController::internal_server_error(error_request),
        NpmErrorKind::SupportedMimeTypeError => {
            DefaultController::internal_server_error(error_request)
        }
        NpmErrorKind::InvalidMimeType => DefaultController::client_error(error_request),
        NpmErrorKind::NotFound => DefaultController::not_found(error_request),
    };

    error_response
}

fn respond_to_request(
    stream: &mut (impl Read + Write),
    router: &router::Router,
    expansions_generator: &dyn ExpansionsAccess,
) -> Result<(), NpmExpansionsError> {
    let request = Request::build(stream)?;
    let response = router.route_request(request, expansions_generator)?;

    stream.write_all(response.as_slice()).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_expansions_model::MockExpansionsModel;
    use crate::mock_tcp_stream::MockTcpStream;
    use crate::npm_controller::ControllerFunction;
    use crate::npm_controller::NpmController;

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

            let mock_generator = &MockExpansionsModel::default();

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )]));

            let response = respond_to_request(&mut stream, &router, mock_generator);

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

            let mock_generator = &MockExpansionsModel::default();

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )]));

            let response = respond_to_request(&mut stream, &router, mock_generator);

            assert!(response.is_ok());
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
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )]));

            let mock_generator = &MockExpansionsModel::default();

            let response = respond_to_request(&mut stream, &router, mock_generator);

            assert!(response.is_err());
        }
    }

    mod connection_handler {
        use crate::npm_controller::ControllerFunction;

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
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )]));

            let mock_generator = &MockExpansionsModel::default();

            let response = handle_connection(&mut stream, &router, mock_generator);

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

            let mock_generator = &MockExpansionsModel::default();

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )]));

            let response = handle_connection(&mut stream, &router, mock_generator);

            assert_eq!(response, ());
        }
    }
}
