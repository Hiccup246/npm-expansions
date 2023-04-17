use crate::default_controller::DefaultController;
use crate::expansions_model::ExpansionsAccess;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::mock_tcp_stream::TcpAddr;
use crate::npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
use crate::router::Router;
use std::{
    collections::HashMap,
    io::{Read, Write},
    sync::{Arc, RwLock},
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
    stream: &mut (impl Read + Write + TcpAddr),
    router: Arc<Router>,
    expansions_model: Arc<RwLock<dyn ExpansionsAccess>>,
) -> Result<(), NpmExpansionsError> {
    let response = respond_to_request(stream, router, expansions_model);

    if let Err(res) = response {
        respond_to_request_error(stream, &res)
    } else {
        Ok(())
    }
}

fn respond_to_request(
    stream: &mut (impl Read + Write + TcpAddr),
    router: Arc<Router>,
    expansions_model: Arc<RwLock<dyn ExpansionsAccess>>,
) -> Result<(), NpmExpansionsError> {
    let request = HttpRequest::build(stream)?;
    let response = router.route_request(&request, expansions_model)?;

    log_request(&request, &response);

    stream
        .write_all(response.into_bytes_vec().as_slice())
        .or(Err(NpmExpansionsError::from(
            NpmErrorKind::InternalServerError,
        )))?;

    Ok(())
}

fn respond_to_request_error(
    stream: &mut (impl Read + Write + TcpAddr),
    error: &NpmExpansionsError,
) -> Result<(), NpmExpansionsError> {
    let http_request = HttpRequest::build(stream).unwrap_or_else(|_error| {
        HttpRequest::new(
            "",
            "",
            HashMap::from([(
                "Accept".to_string(),
                "text/html,application/json".to_string(),
            )]),
            HashMap::new(),
        )
    });

    let error_response = match error.kind() {
        NpmErrorKind::InvalidRequestHeaders => DefaultController::client_error(&http_request),
        NpmErrorKind::TooManyRequestHeaders => DefaultController::client_error(&http_request),
        NpmErrorKind::InternalServerError => {
            DefaultController::internal_server_error(&http_request)
        }
        NpmErrorKind::InvalidHttpRequest => DefaultController::internal_server_error(&http_request),
        NpmErrorKind::InvalidRequestMimeType => DefaultController::client_error(&http_request),
        NpmErrorKind::RouteNotFound => DefaultController::not_found(&http_request),
    }?;

    log_request(&http_request, &error_response);

    stream
        .write_all(error_response.into_bytes_vec().as_slice())
        .or(Err(NpmExpansionsError::from(
            NpmErrorKind::InternalServerError,
        )))?;

    Ok(())
}

fn log_request(request: &HttpRequest, response: &HttpResponse) {
    let referer = request
        .headers()
        .get("Referer")
        .or_else(|| request.headers().get("referer"));
    let user_agent = request
        .headers()
        .get("User-Agent")
        .or_else(|| request.headers().get("user-agent"));

    println!(
        "{} - - [{}] \"{}\" {} {} \"{}\" \"{}\"",
        request.host(),
        chrono::Utc::now().format("%d/%b/%Y:%H:%M:%S %z"),
        request.status_line(),
        response.status_code(),
        response.into_bytes_vec().len(),
        referer.unwrap_or(&"-".to_string()),
        user_agent.unwrap_or(&"-".to_string()),
    );
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

            let mock_expansions_model = Arc::new(RwLock::new(MockExpansionsModel::default()));

            let router = Arc::new(Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )])));

            let response = respond_to_request(&mut stream, router, mock_expansions_model);

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

            let mock_expansions_model = Arc::new(RwLock::new(MockExpansionsModel::default()));

            let router = Arc::new(Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )])));

            let response = respond_to_request(&mut stream, router, mock_expansions_model);

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

            let router = Arc::new(Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )])));

            let mock_expansions_model = Arc::new(RwLock::new(MockExpansionsModel::default()));

            let response = respond_to_request(&mut stream, router, mock_expansions_model);

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

            let router = Arc::new(Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )])));

            let mock_expansions_model = Arc::new(RwLock::new(MockExpansionsModel::default()));

            let response = handle_connection(&mut stream, router, mock_expansions_model);

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

            let mock_expansions_model = Arc::new(RwLock::new(MockExpansionsModel::default()));

            let router = Arc::new(Router::new(HashMap::from([(
                "GET / HTTP/1.1",
                NpmController::random as ControllerFunction,
            )])));

            let response = handle_connection(&mut stream, router, mock_expansions_model);

            assert!(response.is_ok());
        }
    }
}
