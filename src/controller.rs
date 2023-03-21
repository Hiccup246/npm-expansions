use crate::accept_header_handler;
use crate::npm_expansion_error::NpmErrorKind;
use crate::response::Response;
use crate::NpmExpansions;
use crate::Request;
use std::collections::HashMap;
use std::fs;

pub struct Controller {}

fn ok(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}")
        .as_bytes()
        .to_vec()
}

fn not_acceptable(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 406 Not Acceptable\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{contents}").as_bytes().to_vec()
}

fn not_found_route(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {length}\r\n\r\n{contents}")
        .as_bytes()
        .to_vec()
}

fn internal(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: {length}\r\n\r\n{contents}")
        .as_bytes()
        .to_vec()
}

fn bad_req(contents: String) -> Vec<u8> {
    let length = contents.len();
    format!("HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {length}\r\n\r\n{contents}")
        .as_bytes()
        .to_vec()
}

impl Controller {
    pub fn index(request: &Request) -> Vec<u8> {
        let best = accept_header_handler::best_match(
            Vec::from(["text/html", "text/css", "text/javascript"]),
            request.headers().get("Accept").unwrap(),
        );

        let mut response: Vec<u8> = Vec::new();

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "text/html" => Response::new(
                    "200 OK",
                    "Content-Type: text/html",
                    fs::read_to_string("pages/npm_expansions/npm_expansions.html").unwrap(),
                ),
                "text/css" => Response::new(
                    "200 OK",
                    "Content-Type: text/css",
                    fs::read_to_string("pages/npm_expansions/npm_expansions.css").unwrap(),
                ),
                "text/javascript" => Response::new(
                    "200 OK",
                    "Content-Type: text/javascript",
                    fs::read_to_string("pages/npm_expansions/npm_expansions.js").unwrap(),
                ),
                _ => Response::new(
                    "406 Not Acceptable",
                    "",
                    format!("Please accept text/html, text/css or text/javascript"),
                ),
            }
            .to_vec();
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error"),
            };
        };

        response
    }

    pub fn random(request: &Request) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => Response::new(
                    "200 OK",
                    "Content-Type: application/json",
                    format!(
                        "{{\"npm-expansion\": \"{}\"}}",
                        NpmExpansions::random_expansion()
                    ),
                ),
                _ => Response::new(
                    "406 NOT ACCEPTABLE",
                    "",
                    format!("Please accept application/json"),
                ),
            }
            .to_vec();
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error"),
            };
        };

        response
    }

    pub fn not_found(request: &Request) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => Response::new("404 NOT FOUND", "", format!("NOT FOUND")),
                "text/html" => Response::new(
                    "404 NOT FOUND",
                    "",
                    fs::read_to_string("pages/not_found/not_found.html").unwrap(),
                ),
                _ => Response::new(
                    "406 NOT ACCEPTABLE",
                    "",
                    format!("Please accept application/json or text/html"),
                ),
            }
            .to_vec();
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error"),
            };
        };

        response
    }

    pub fn internal_server_error(request: &Request) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => Response::new(
                    "500 INTERNAL SERVER ERROR",
                    "",
                    format!("INTERNAL SERVER ERROR"),
                ),
                "text/html" => Response::new(
                    "500 INTERNAL SERVER ERROR",
                    "Content-Type: text/html",
                    fs::read_to_string("pages/internal_server_error/internal_server_error.html")
                        .unwrap(),
                ),
                _ => Response::new(
                    "406 NOT ACCEPTABLE",
                    "",
                    format!("Please accept application/json or text/html"),
                ),
            }
            .to_vec()
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error"),
            };
        };

        response
    }

    pub fn client_error(request: &Request) -> Vec<u8> {
        let mut response: Vec<u8> = Vec::new();

        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        );

        if let Ok(best_match) = best {
            response = match best_match.as_str() {
                "application/json" => Response::new("400 BAD REQUEST", "", format!("BAD REQUEST")),
                "text/html" => Response::new(
                    "400 BAD REQUEST",
                    "Content-Type: text/html",
                    fs::read_to_string("pages/client_error/client_error.html").unwrap(),
                ),
                _ => Response::new(
                    "406 NOT ACCEPTABLE",
                    "",
                    format!("Please accept application/json or text/html"),
                ),
            }
            .to_vec();
        } else if let Err(err) = best {
            response = match err.kind() {
                NpmErrorKind::SupportedMimeTypeError => Controller::internal_server_error(request),
                NpmErrorKind::InvalidHeader => Controller::client_error(request),
                _ => panic!("Unknown error"),
            };
        };

        response
    }

    pub fn static_file(request: &Request) -> Vec<u8> {
        let split_status_line: Vec<&str> = request.status_line().split(" ").collect();
        let file_name = split_status_line.get(1);

        let response = if let Some(name) = file_name {
            let extension = name.split(".").last().unwrap_or("");

            let content_type = match extension {
                "png" => "\r\nContent-Type: image/png",
                "ico" => "\r\nContent-Type: image/vnd.microsoft.icon",
                "xml" => "\r\nContent-Type: application/xml",
                "txt" => "\r\nContent-Type: text/plain",
                _ => "",
            };

            let file_path = format!("static{name}");
            let mut contents = fs::read(file_path).unwrap();
            let length = contents.len();

            let mut response =
                format!("HTTP/1.1 200 OK\r\nContent-Length: {length}{content_type}\r\n\r\n")
                    .as_bytes()
                    .to_vec();

            response.append(&mut contents);

            response
        } else {
            Controller::client_error(request)
        };

        response
    }
}
