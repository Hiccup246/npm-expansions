use crate::accept_header_handler;
use crate::NpmExpansions;
use crate::Request;
use std::fs;
use crate::npm_expansion_error::NpmErrorKind;

pub struct Controller {}

fn ok(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes().to_vec()
}

fn not_acceptable(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 406 Not Acceptable\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}").as_bytes().to_vec()
}

fn not_found_route(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes().to_vec()
}

fn internal(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes().to_vec()
}

fn bad_req(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes().to_vec()
}

impl Controller {
    pub fn index(request: &Request) -> Vec<u8> {
        let mut response:Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["text/html", "text/css", "text/javascript"]),
            request.headers().get("Accept").unwrap(),
        );
        
        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "text/html" => ok(fs::read_to_string("pages/npm_expansions/npm_expansions.html").unwrap()),
                "text/css" => ok(fs::read_to_string("pages/npm_expansions/npm_expansions.css").unwrap()),
                "text/javascript" => ok(fs::read_to_string("pages/npm_expansions/npm_expansions.js").unwrap()),
                _ => not_acceptable(format!("Please accept text/html, text/css or text/javascript")),
            }
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error")
            };
        };

        response
    }

    pub fn random(request: &Request) -> Vec<u8> {
        let mut response:Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => ok(format!("{{\"npm-expansion\": \"{}\"}}", NpmExpansions::random_expansion())),
                _ => not_acceptable(format!("Please accept application/json")),
            }
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error")
            };
        };

        response
    }

    pub fn not_found(request: &Request) -> Vec<u8> {
        let mut response:Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => not_found_route(format!("NOT FOUND")),
                "text/html" => not_found_route(fs::read_to_string("pages/not_found/not_found.html").unwrap()),
                _ => not_acceptable(format!("Please accept application/json or text/html")),
            }
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error")
            };
        };

        response
    }

    pub fn internal_server_error(request: &Request) -> Vec<u8> {
        let mut response:Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => internal(format!("INTERNAL SERVER ERROR")),
                "text/html" => internal(fs::read_to_string("pages/internal_server_error/internal_server_error.html").unwrap()),
                _ => not_acceptable(format!("Please accept application/json or text/html")),
            }
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error")
            };
        };

        response
    }

    pub fn client_error(request: &Request) -> Vec<u8> {
        let mut response:Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => bad_req(format!("BAD REQUEST")),
                "text/html" => bad_req(fs::read_to_string("pages/client_error/client_error.html").unwrap()),
                _ => not_acceptable(format!("Please accept application/json or text/html")),
            }
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error")
            };
        };

        response
    }

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
