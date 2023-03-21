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
    let route_config: HashMap<&str, fn(&Request) -> Vec<u8>> = HashMap::from([
        (
            "GET / HTTP/1.1",
            Controller::index as fn(&Request) -> Vec<u8>,
        ),
        (
            "GET /random HTTP/1.1",
            Controller::random as fn(&Request) -> Vec<u8>,
        ),
        ("404", Controller::not_found as fn(&Request) -> Vec<u8>),
    ]);
    let router = router::Router::new(route_config);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle_connection(stream);
        new_connection_handler(stream, &router);
    }
}

// If any error occurs then we should just render 500
fn new_connection_handler(mut stream: TcpStream, router: &router::Router) {
    let request = Request::build(&stream);
    let response = router.route_request(request).unwrap();
    stream.write_all(response.as_slice()).unwrap();
}

fn serve_static_file(status_line: String, mut stream: TcpStream) {
    let split_status_line: Vec<&str> = status_line.split(" ").collect();
    let file_name = split_status_line.get(1).unwrap();
    let extension = file_name.split(".").last().unwrap();

    let content_type = match extension {
        "png" => "image/png",
        "ico" => "image/vnd.microsoft.icon",
        "xml" => "application/xml",
        "txt" => "text/plain",
        _ => "",
    };

    let file_path = format!("static{file_name}");
    let contents = fs::read(file_path).unwrap();
    let length = contents.len();
    let response: String;

    if content_type.is_empty() {
        response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n");
    } else {
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n"
        );
    }

    stream.write(response.as_bytes()).unwrap();
    stream.write(&contents).unwrap();
}

fn match_static(status_line: &str) -> bool {
    let static_file_names: Vec<String> = directory_file_names("static".to_string())
        .iter()
        .map(|directory_name| format!("GET /{directory_name} HTTP/1.1"))
        .collect();

    static_file_names.contains(&status_line.to_string())
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
