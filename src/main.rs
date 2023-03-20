use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod accept_header_handler;
mod mime_type_parser;
mod npm_expansions;
mod router;
mod request;

pub use request::Request;
pub use crate::npm_expansions::NpmExpansions;


trait ControllerFunction {

}

pub struct Controller {

}

impl Controller {
    pub fn index<'a>(request: &Request) -> &'a [u8] {
        "HTTP/1.1 200 OK".as_bytes()
    }

    pub fn random<'a>(request: &Request) -> &'a [u8] {
        "HTTP/1.1 200 OK".as_bytes()
    }

    pub fn not_found(request: &Request) {
        
    }

    pub fn internal_server_error(request: &Request) {
        
    }
}

impl ControllerFunction for Controller {

}

fn main() {
    let route_config:HashMap<&str,fn(&Request) -> &[u8]> = HashMap::from([
        ("GET / HTTP/1.1", Controller::index as fn(&Request) -> &[u8]),
        ("GET /random HTTP/1.1", Controller::random as fn(&Request) -> &[u8])
    ]);
    let router = router::Router::new(route_config);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle_connection(stream);
        new_connection_handler(stream, &router);
    }
}

fn new_connection_handler(mut stream: TcpStream, router: &router::Router) {
    let request = Request::new(&stream);
    let response = router.route_request(request);

    stream.write_all(response);
}

fn handle_root_route(mut stream: TcpStream, request_headers: HashMap<String, String>) {
    let response;

    // If request accepts application/json then we are good to go
    let best = accept_header_handler::best_match(
        Vec::from(["text/html", "text/css", "text/javascript"]),
        request_headers.get("Accept").unwrap(),
    )
    .unwrap();

    if best == "text/html" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("pages/npm_expansions/npm_expansions.html").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    } else if best == "text/css" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("pages/npm_expansions/npm_expansions.css").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    } else if best == "text/javascript" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("pages/npm_expansions/npm_expansions.js").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    } else {
        let status_line = "HTTP/1.1 406 Not Acceptable";
        let contents = format!("Please accept text/html");
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    }

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_not_found(mut stream: TcpStream, request_headers: HashMap<String, String>) {
    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let response;

    // If request accepts application/json then we are good to go
    let best = accept_header_handler::best_match(
        Vec::from(["application/json", "text/html"]),
        request_headers.get("Accept").unwrap(),
    )
    .unwrap();

    if best == "application/json" {
        response = format!("{status_line}\r\n\r\n");
    } else if best == "text/html" {
        let contents = fs::read_to_string("pages/not_found/not_found.html").unwrap();
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    } else {
        status_line = "HTTP/1.1 406 Not Acceptable";
        let contents = format!("Please accept application/json or text/html");
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    }

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_random_route(mut stream: TcpStream, request_headers: HashMap<String, String>) {
    let response: String;
    let best = accept_header_handler::best_match(
        Vec::from(["application/json"]),
        request_headers.get("Accept").unwrap(),
    )
    .unwrap();

    if best == "application/json" {
        let status_line = "HTTP/1.1 200 OK";
        let expansion = NpmExpansions::random_expansion();
        let contents = format!("{{\"npm-expansion\": \"{expansion}\"}}");
        let length = contents.len();
        let content_type = "application/json";

        response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}\r\n")
    } else {
        let status_line = "HTTP/1.1 406 Not Acceptable";
        let contents = format!("Please accept application/json");
        let length = contents.len();

        response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    }

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut buffer = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty());

    let request_line = buffer.next().unwrap();

    let mut request_structure: HashMap<String, String> = HashMap::new();

    for header in buffer {
        let (key, value) = header.split_at(header.find(":").unwrap());
        let (_colon, header_value) = value.split_at(1);
        request_structure.insert(key.to_string(), header_value.trim().to_string());
    }

    match request_line.as_str() {
        "GET / HTTP/1.1" => handle_root_route(stream, request_structure),
        "GET /random HTTP/1.1" => handle_random_route(stream, request_structure),
        status_line if match_static(status_line) => {
            serve_static_file(status_line.to_string(), stream)
        }
        &_ => handle_not_found(stream, request_structure),
    };
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
