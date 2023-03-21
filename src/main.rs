use std::{
    collections::HashMap,
    fs,
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

pub use crate::npm_expansions::NpmExpansions;
pub use controller::Controller;
use npm_expansion_error::NpmErrorKind;
pub use request::Request;

fn main() {
    let router = router::Router::new(route_config());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        connection_handler(stream, &router);
    }
}

// If any error occurs then we should just render 500
fn connection_handler(mut stream: TcpStream, router: &router::Router) {
    let request = Request::build(&stream);

    if let Ok(req) = request {
        let response = router.route_request(req);

        if let Ok(response) = response {
            stream.write_all(response.as_slice()).unwrap();
        } else {
            stream
                .write_all(
                    Controller::internal_server_error(&Request::build(&stream).unwrap()).as_slice(),
                )
                .unwrap();
        }
    } else if let Err(error) = request {
        let response = match error.kind() {
            NpmErrorKind::RequestParseError => Controller::internal_server_error(&Request::new(
                "",
                HashMap::from([(
                    "Accept".to_string(),
                    "text/html,application/json".to_string(),
                )]),
            )),
            NpmErrorKind::InvalidHeader => Controller::client_error(&Request::new(
                "",
                HashMap::from([(
                    "Accept".to_string(),
                    "text/html,application/json".to_string(),
                )]),
            )),
            NpmErrorKind::TooManyHeaders => Controller::client_error(&Request::new(
                "",
                HashMap::from([(
                    "Accept".to_string(),
                    "text/html,application/json".to_string(),
                )]),
            )),
            _ => panic!("Unknown error"),
        };

        stream.write_all(response.as_slice()).unwrap();
    }
}

fn route_config() -> HashMap<String, fn(&Request) -> Vec<u8>> {
    let mut config: HashMap<String, fn(&Request) -> Vec<u8>> = HashMap::from([
        (
            "GET / HTTP/1.1".to_string(),
            Controller::index as fn(&Request) -> Vec<u8>,
        ),
        (
            "GET /random HTTP/1.1".to_string(),
            Controller::random as fn(&Request) -> Vec<u8>,
        ),
        (
            "404".to_string(),
            Controller::not_found as fn(&Request) -> Vec<u8>,
        ),
    ]);

    let static_file_names: Vec<String> = directory_file_names("static".to_string())
        .iter()
        .map(|file_name| format!("GET /{file_name} HTTP/1.1"))
        .collect();

    for static_file in static_file_names {
        config.insert(
            static_file,
            Controller::static_file as fn(&Request) -> Vec<u8>,
        );
    }

    config
}

fn directory_file_names(directory_path: String) -> Vec<String> {
    let directory = fs::read_dir(directory_path).unwrap();
    directory
        .map(|dir_entry| dir_entry.unwrap())
        .filter(|dir_entry| dir_entry.file_type().unwrap().is_file())
        .map(|dir_entry| dir_entry.file_name().into_string().unwrap())
        .collect()
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
