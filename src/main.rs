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
mod npm_expansions;
mod request;
mod router;

pub use crate::npm_expansions::NpmExpansions;
pub use controller::Controller;
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
    let response = router.route_request(request).unwrap();
    stream.write_all(response.as_slice()).unwrap();
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

fn directory_file_names(directoryPath: String) -> Vec<String> {
    let directory = fs::read_dir(directoryPath).unwrap();
    directory
        .map(|dirEntry| dirEntry.unwrap())
        .filter(|dirEntry| dirEntry.file_type().unwrap().is_file())
        .map(|dirEntry| dirEntry.file_name().into_string().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_server_and_connection() -> (TcpListener, TcpStream) {
        (
            TcpListener::bind("127.0.0.1:7878").unwrap(),
            TcpStream::connect("127.0.0.1:7878").unwrap(),
        )
    }

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
