use crate::accept_header_handler;
use crate::NpmExpansions;
use crate::Request;
use std::fs;

pub struct Controller {}

impl Controller {
    pub fn index(request: &Request) -> Vec<u8> {
        let response;

        // If request accepts application/json then we are good to go
        let best = accept_header_handler::best_match(
            Vec::from(["text/html", "text/css", "text/javascript"]),
            request.headers().get("Accept").unwrap(),
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

        response.as_bytes().to_vec()
    }

    pub fn random<'a>(request: &Request) -> Vec<u8> {
        let response: String;
        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            request.headers().get("Accept").unwrap(),
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

        response.as_bytes().to_vec()
    }

    pub fn not_found(request: &Request) -> Vec<u8> {
        let mut status_line = "HTTP/1.1 404 NOT FOUND";
        let response;

        // If request accepts application/json then we are good to go
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
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

        response.as_bytes().to_vec()
    }

    pub fn internal_server_error(request: &Request) {}

    pub fn static_file(request: &Request) -> Vec<u8> {
        let split_status_line: Vec<&str> = request.status_line().split(" ").collect();
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
        let mut contents = fs::read(file_path).unwrap();
        let length = contents.len();
        let string_response: String;

        if content_type.is_empty() {
            string_response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n");
        } else {
            string_response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n"
            );
        }

        let mut response = string_response.as_bytes().to_vec();

        response.append(&mut contents);

        response
    }
}
