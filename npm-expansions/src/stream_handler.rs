use crate::default_controller::DefaultController;
use crate::expansions_model::ExpansionsAccess;
use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use crate::request::Request;
use crate::router;
use std::{
    collections::HashMap,
    io::{Read, Write},
};

/// Handles a http stream by building a request object and routing the request to a controller. The controllers response
/// is then written to the stream. If the produced response is a failure then a failure response
/// is attempted using the DefaultController i.e. 400, 500, 404 etc. If the fail response
/// produces a error then the error is returned as this indicates a fatal server error.
///
/// # Arguments
///
/// * `stream` - An incoming TCP stream
/// * `router` - A Router object which can route a stream to a controller
/// * `expansions_model` - A ExpansionsModel which produces NPM expansions and acts as persistent database
///
pub fn handle_connection(
    stream: &mut (impl Read + Write),
    router: &router::Router,
    expansions_model: &dyn ExpansionsAccess,
) -> Result<(), NpmExpansionsError> {
    let response = respond_to_request(stream, router, expansions_model);

    if let Err(res) = response {
        respond_to_request_error(stream, &res)
    } else {
        Ok(())
    }
}

fn respond_to_request(
    stream: &mut (impl Read + Write),
    router: &router::Router,
    expansions_model: &dyn ExpansionsAccess,
) -> Result<(), NpmExpansionsError> {
    let request = Request::build(stream)?;
    let response = router.route_request(request, expansions_model)?;

    stream
        .write_all(response.as_slice())
        .or(Err(NpmExpansionsError::from(
            NpmErrorKind::InternalServerError,
        )))?;

    Ok(())
}

fn respond_to_request_error(
    stream: &mut (impl Read + Write),
    error: &NpmExpansionsError,
) -> Result<(), NpmExpansionsError> {
    let error_request = Request::new(
        "",
        HashMap::from([(
            "Accept".to_string(),
            "text/html,application/json".to_string(),
        )]),
        HashMap::new(),
    );

    let error_response = match error.kind() {
        NpmErrorKind::InvalidHeader => DefaultController::client_error(&error_request),
        NpmErrorKind::TooManyHeaders => DefaultController::client_error(&error_request),
        NpmErrorKind::InternalServerError => {
            DefaultController::internal_server_error(&error_request)
        }
        NpmErrorKind::RequestParseError => DefaultController::internal_server_error(&error_request),
        NpmErrorKind::SupportedMimeTypeError => {
            DefaultController::internal_server_error(&error_request)
        }
        NpmErrorKind::InvalidMimeType => DefaultController::client_error(&error_request),
        NpmErrorKind::NotFound => DefaultController::not_found(&error_request),
    }?;

    stream
        .write_all(error_response.as_slice())
        .or(Err(NpmExpansionsError::from(
            NpmErrorKind::InternalServerError,
        )))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_expansions_model::MockExpansionsModel;
    use crate::mock_tcp_stream::MockTcpStream;
    use crate::npm_controller::{ControllerFunction, NpmController};

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

            assert!(response.is_ok());
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

            assert!(response.is_ok());
        }
    }
}
