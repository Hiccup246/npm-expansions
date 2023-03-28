use crate::accept_header_handler;
use crate::npm_expansion_error::NpmErrorKind;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::response::Response;
use crate::NpmExpansions;
use crate::Request;
use levenshtein::levenshtein;
use std::fs;

pub struct Controller {}

impl Controller {
    /// Returns a vector byte representation of the npm_expansions page including html, css and javascript
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET / HTTP/1.1", HashMap::from([("Accept".to_string(), "text/html".to_string())]))
    /// let response = Controller::index(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET / HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::index(&request)
    /// ```
    pub fn index(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["text/html", "text/css", "text/javascript"]),
            accept_header.unwrap_or(&"".to_string()),
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
                "Please accept text/html, text/css or text/javascript".to_string(),
            ),
        }
        .into_http_response();

        Ok(response)
    }

    /// Returns a vector byte representation of a json object containing a random npm expansion
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]))
    /// let response = Controller::random(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::random(&request)
    /// ```
    pub fn random(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => Response::new(
                "200 OK",
                "Content-Type: application/json",
                format!(
                    "{{\"npmExpansion\": \"{}\"}}",
                    NpmExpansions::random_expansion()
                ),
            ),
            _ => Response::new(
                "406 NOT ACCEPTABLE",
                "",
                "Please accept application/json".to_string(),
            ),
        }
        .into_http_response();

        Ok(response)
    }

    /// Returns a vector byte representation of a json array containing all npm expansions
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]))
    /// let response = Controller::all(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::all(&request)
    /// ```
    pub fn all(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let string_expansions: Vec<String> = NpmExpansions::expansions()
            .iter()
            .map(|expansions| format!("\"{expansions}\""))
            .collect();

        let response = match best.as_str() {
            "application/json" => Response::new(
                "200 OK",
                "Content-Type: application/json",
                format!("[{}]", string_expansions.join(",")),
            ),
            _ => Response::new(
                "406 NOT ACCEPTABLE",
                "",
                "Please accept application/json".to_string(),
            ),
        }
        .into_http_response();

        Ok(response)
    }

    /// Returns a vector byte representation of a json array containing the top ten matches of npm expansions given
    /// a request with a search_query query param
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]))
    /// let response = Controller::search(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::search(&request)
    /// ```
    pub fn search(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let mut weighted_matched: Vec<(usize, &str)> = NpmExpansions::expansions()
            .iter()
            .map(|expansion| (levenshtein(expansion, "Neo post manager"), *expansion))
            .collect();

        weighted_matched.sort_by(|a, b| a.0.cmp(&b.0));

        let top_ten: Vec<String> = weighted_matched[0..10]
            .iter()
            .map(|expansions| format!("\"{}\"", expansions.1))
            .collect();

        let response = match best.as_str() {
            "application/json" => Response::new(
                "200 OK",
                "Content-Type: application/json",
                format!("[{}]", top_ten.join(",")),
            ),
            _ => Response::new(
                "406 NOT ACCEPTABLE",
                "",
                "Please accept application/json".to_string(),
            ),
        }
        .into_http_response();

        Ok(response)
    }

    /// Returns a vector byte representation of the not_found page including html, css and javascript
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]))
    /// let response = Controller::not_found(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::not_found(&request)
    /// ```
    pub fn not_found(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => Response::new("404 NOT FOUND", "", "NOT FOUND".to_string()),
            "text/html" => Response::new(
                "404 NOT FOUND",
                "",
                fs::read_to_string("pages/not_found/not_found.html").unwrap(),
            ),
            _ => Response::new(
                "406 NOT ACCEPTABLE",
                "",
                "Please accept application/json or text/html".to_string(),
            ),
        }
        .into_http_response();

        Ok(response)
    }

    /// Returns a vector byte representation of the internal_server_error page including html, css and javascript
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]))
    /// let response = Controller::internal_server_error(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET / HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::internal_server_error(&request)
    /// ```
    pub fn internal_server_error(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => Response::new(
                "500 INTERNAL SERVER ERROR",
                "",
                "INTERNAL SERVER ERROR".to_string(),
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
                "Please accept application/json or text/html".to_string(),
            ),
        }
        .into_http_response();

        Ok(response)
    }

    /// Returns a vector byte representation of the client_error page including html, css and javascript
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]))
    /// let response = Controller::client_error(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET / HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::client_error(&request)
    /// ```
    pub fn client_error(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => Response::new("400 BAD REQUEST", "", "BAD REQUEST".to_string()),
            "text/html" => Response::new(
                "400 BAD REQUEST",
                "Content-Type: text/html",
                fs::read_to_string("pages/client_error/client_error.html").unwrap(),
            ),
            _ => Response::new(
                "406 NOT ACCEPTABLE",
                "",
                "Please accept application/json or text/html".to_string(),
            ),
        }
        .into_http_response();

        Ok(response)
    }

    /// Returns a vector byte representation of a static asset located within this projects static directory
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]))
    /// let response = Controller::static_file(&request);
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// let request = Request::new("GET / HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]))
    /// Controller::static_file(&request)
    /// ```
    pub fn static_file(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let split_status_line: Vec<&str> = request.status_line().split(' ').collect();
        let file_name = split_status_line.get(1).ok_or(NpmExpansionsError::new(
            NpmErrorKind::InvalidRequestStatusLine,
        ))?;
        let extension = file_name.split('.').last().unwrap_or("");

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(Controller::index; "index")]
    #[test_case(Controller::random; "random")]
    #[test_case(Controller::all; "all")]
    #[test_case(Controller::search; "search")]
    #[test_case(Controller::not_found; "not_found")]
    #[test_case(Controller::internal_server_error; "internal_server_error")]
    #[test_case(Controller::client_error; "client_error")]

    fn valid_request(controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>) {
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        assert!(controller_function(&request).is_ok())
    }

    #[test_case(Controller::index; "index")]
    #[test_case(Controller::random; "random")]
    #[test_case(Controller::all; "all")]
    #[test_case(Controller::search; "search")]
    #[test_case(Controller::not_found; "not_found")]
    #[test_case(Controller::internal_server_error; "internal_server_error")]
    #[test_case(Controller::client_error; "client_error")]
    fn invalid_request_headers(
        controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
    ) {
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/".to_string())]),
            HashMap::new(),
        );

        assert!(controller_function(&request).is_err())
    }

    #[test_case(Controller::index; "index")]
    #[test_case(Controller::random; "random")]
    #[test_case(Controller::not_found; "not_found")]
    #[test_case(Controller::internal_server_error; "internal_server_error")]
    #[test_case(Controller::client_error; "client_error")]

    fn lower_case_accept_header(
        controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
    ) {
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        assert!(controller_function(&request).is_ok())
    }

    #[test_case(Controller::index; "index")]
    #[test_case(Controller::random; "random")]
    #[test_case(Controller::not_found; "not_found")]
    #[test_case(Controller::internal_server_error; "internal_server_error")]
    #[test_case(Controller::client_error; "client_error")]

    fn no_accept_header(controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>) {
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());

        assert!(controller_function(&request).is_ok())
    }

    mod static_file_tests {
        use super::*;

        #[test]
        fn valid_static_file() {
            let request = Request::new(
                "GET /robots.txt HTTP/1.1",
                HashMap::from([("Accept".to_string(), "text/plain".to_string())]),
                HashMap::new(),
            );

            assert!(Controller::static_file(&request).is_ok())
        }

        #[test]
        fn invalid_status_line() {
            let request = Request::new(
                "GET",
                HashMap::from([("Accept".to_string(), "text/plain".to_string())]),
                HashMap::new(),
            );

            assert!(Controller::static_file(&request).is_err())
        }

        #[test]
        #[should_panic(expected = "No such file or directory")]
        fn invalid_static_file() {
            let request = Request::new(
                "GET /nothing HTTP/1.1",
                HashMap::from([("Accept".to_string(), "text/plain".to_string())]),
                HashMap::new(),
            );

            assert!(Controller::static_file(&request).is_err())
        }
    }
}
