use crate::accept_header_handler;
use crate::npm_expansion_error::NpmErrorKind;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::response::Response;
use crate::NpmExpansions;
use crate::Request;
use std::fs;

pub struct Controller {}

impl Controller {
    pub fn index(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let best = accept_header_handler::best_match(
            Vec::from(["text/html", "text/css", "text/javascript"]),
            request.headers().get("Accept").unwrap(),
        )?;

        let response = match best.as_str() {
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
        .into_http_response();

        Ok(response)
    }

    pub fn random(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            request.headers().get("Accept").unwrap(),
        )?;

        let response = match best.as_str() {
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
        .into_http_response();

        Ok(response)
    }

    pub fn not_found(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        )?;

        let response = match best.as_str() {
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
        .into_http_response();

        Ok(response)
    }

    pub fn internal_server_error(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        )?;

        let response = match best.as_str() {
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
        .into_http_response();

        Ok(response)
    }

    pub fn client_error(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            request.headers().get("Accept").unwrap(),
        )?;

        let response = match best.as_str() {
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
        .into_http_response();

        Ok(response)
    }

    pub fn static_file(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let split_status_line: Vec<&str> = request.status_line().split(" ").collect();
        let file_name = split_status_line.get(1).ok_or(NpmExpansionsError::new(
            NpmErrorKind::InvalidRequestStatusLine,
        ))?;
        let extension = file_name.split(".").last().unwrap_or("");

        let content_type = match extension {
            "png" => "\r\nContent-Type: image/png",
            "ico" => "\r\nContent-Type: image/vnd.microsoft.icon",
            "xml" => "\r\nContent-Type: application/xml",
            "txt" => "\r\nContent-Type: text/plain",
            _ => "",
        };

        let file_path = format!("static{file_name}");
        let mut contents = fs::read(file_path).unwrap();
        let length = contents.len();

        let mut response =
            format!("HTTP/1.1 200 OK\r\nContent-Length: {length}{content_type}\r\n\r\n")
                .as_bytes()
                .to_vec();

        response.append(&mut contents);

        Ok(response)
    }
}
