use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};

mod expansions_generator;
pub use crate::expansions_generator::NpmExpansionGenerator;

fn main() {
    // NpmExpansionsGenerator::convert_text_file();
    // fs::read_to_string("expansions.txt").unwrap();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_root_route(mut stream: TcpStream, request_headers: HashMap<String, String>) {
    let mut status_line = "HTTP/1.1 200 OK";
    let response;
    
    // If request accepts text/html then we are good to go
    if true {
        let contents = fs::read_to_string("npm_expansions.html").unwrap();
        let length = contents.len();

        response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    } else {
        status_line = "HTTP/1.1 406 Not Acceptable";
        let contents = format!("Please accept text/html");
        let length = contents.len();
        response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    }

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_not_found(mut stream: TcpStream, request_headers: HashMap<String, String>) {
    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let response;

    // If request accepts application/json then we are good to go
    if *request_headers.get("Content-Type").unwrap_or(&"".to_string()) == "application/json".to_string() {
        response = format!(
            "{status_line}\r\n\r\n"
        );
    // If request accepts text/html then we are good to go
    } else if *request_headers.get("Content-Type").unwrap_or(&"".to_string()) == "application/json".to_string() {
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    // If request does not accept any types return 406
    } else {
        status_line = "HTTP/1.1 406 Not Acceptable";
        let contents = format!("Please accept application/json");
        let length = contents.len();
        response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
    }

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_random_route(mut stream: TcpStream, request_headers: HashMap<String, String>) {
    let mut status_line = "HTTP/1.1 200 OK";
    let response;

    // If request accepts application/json then we are good to go
    if true {
        let expansion = NpmExpansionGenerator::random_expansion();
        let contents = format!("{{\"npm-expansion\": \"{expansion}\"}}");
        let length = contents.len();
        let content_type = "application/json";

        response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}\r\n"
        );
    } else {
        status_line = "HTTP/1.1 406 Not Acceptable";
        let contents = format!("Please accept application/json");
        let length = contents.len();
        response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
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

    let mut request_structure:HashMap<String, String> = HashMap::new();

    for header in buffer {
        let (key, value) = header.split_at(header.find(":").unwrap() + 1);
        let (_colon, header_value) = value.split_at(1);
        request_structure.insert(key.to_string(), header_value.trim().to_string());
    };


    if request_line == "GET / HTTP/1.1" {
        handle_root_route(stream, request_structure)
    } else if request_line == "GET /random HTTP/1.1" {
        handle_random_route(stream, request_structure)
    } else {
        handle_not_found(stream, request_structure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_server_and_connection() -> (TcpListener, TcpStream) {
        (TcpListener::bind("127.0.0.1:7878").unwrap(), TcpStream::connect("127.0.0.1:7878").unwrap())
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

    #[test]
    fn handle_connection_non_real_route() {
        let (mut listener, mut connection) = setup_server_and_connection();
        let incoming_stream = listener.incoming().next().unwrap().unwrap();
        connection.write("GET /example-route HTTP/1.1 \n \r".as_bytes()).unwrap();    

        handle_connection(incoming_stream);

        let buf_reader = BufReader::new(&mut connection);
        let http_request: Vec<String> = buf_reader
            .lines()
            .filter(|line| line.is_ok())
            .map(|result| result.unwrap())
            .collect();
        
        assert_eq!(format!("{:?}", http_request), "[\"HTTP/1.1 404 NOT FOUND\", \"\"]");

        connection.flush().unwrap();
        connection.shutdown(std::net::Shutdown::Both).unwrap();
    }
}