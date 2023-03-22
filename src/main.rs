use std::{
    collections::HashMap,
    io::prelude::*,
    net::{TcpListener, TcpStream},
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
    let router = router::Router::new(routes_config::route_config());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        connection_handler(stream, &router);
    }
}

fn connection_handler(mut stream: TcpStream, router: &router::Router) {
    let response = respond_to_request(&mut stream, router);

    if let Err(res) = response {
        let error_request = Request::new(
            "",
            HashMap::from([(
                "Accept".to_string(),
                "text/html,application/json".to_string(),
            )]),
        );

        let error_response = match res.kind() {
            NpmErrorKind::InvalidHeader => Controller::client_error(&error_request),
            NpmErrorKind::TooManyHeaders => Controller::client_error(&error_request),
            NpmErrorKind::InternalServerError => Controller::internal_server_error(&error_request),
            NpmErrorKind::RequestParseError => Controller::internal_server_error(&error_request),
            NpmErrorKind::SupportedMimeTypeError => {
                Controller::internal_server_error(&error_request)
            }
        };

        stream
            .write_all(error_response.as_slice()).unwrap()
    }
}

fn respond_to_request(
    mut stream: &TcpStream,
    router: &router::Router,
) -> Result<(), NpmExpansionsError> {
    let request = Request::build(stream)?;
    let response = router.route_request(request)?;

    stream.write_all(response.as_slice()).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn setup_server_and_connection() -> (TcpListener, TcpStream) {
    //     (
    //         TcpListener::bind("127.0.0.1:7878").unwrap(),
    //         TcpStream::connect("127.0.0.1:7878").unwrap(),
    //     )
    // }

    // #[test]
    // fn handle_connection_get_root() {
    //     let (mut listener, mut connection) = setup_server_and_connection();
    //     let incoming_stream = listener.incoming().next().unwrap().unwrap();
    //     connection.write("GET \n \r".as_bytes()).unwrap();

    //     handle_connection(incoming_stream);

    //     let buf_reader = BufReader::new(&mut connection);
    //     let http_request: Vec<String> = buf_reader
    //         .lines()
    //         .filter(|line| line.is_ok())
    //         .map(|result| result.unwrap())
    //         .collect();

    //     assert_eq!(format!("{:?}", http_request), "[\"HTTP/1.1 200 OK\", \"Content-Length: 26\", \"\", \"{\\\"npm-expansion\\\": \\\"hello\\\"}\"]");

    //     connection.flush().unwrap();
    //     connection.shutdown(std::net::Shutdown::Both).unwrap();
    // }

    // #[test]
    // fn handle_connection_non_real_route() {
    //     let (mut listener, mut connection) = setup_server_and_connection();
    //     let incoming_stream = listener.incoming().next().unwrap().unwrap();
    //     connection
    //         .write("GET /example-route HTTP/1.1 \n \r".as_bytes())
    //         .unwrap();

    //     handle_connection(incoming_stream);

    //     let buf_reader = BufReader::new(&mut connection);
    //     let http_request: Vec<String> = buf_reader
    //         .lines()
    //         .filter(|line| line.is_ok())
    //         .map(|result| result.unwrap())
    //         .collect();

    //     assert_eq!(
    //         format!("{:?}", http_request),
    //         "[\"HTTP/1.1 404 NOT FOUND\", \"\"]"
    //     );

    //     connection.flush().unwrap();
    //     connection.shutdown(std::net::Shutdown::Both).unwrap();
    // }
}
