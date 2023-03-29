use crate::accept_header_handler;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::response::Response;
use crate::NpmExpansions;
use crate::Request;
use levenshtein::levenshtein;

pub struct NpmController {}

impl NpmController {
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
    /// let response = NpmController::random(&request);
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
    /// NpmController::random(&request)
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
            _ => not_acceptable_response(),
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
    /// let response = NpmController::all(&request);
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
    /// NpmController::all(&request)
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
            _ => not_acceptable_response(),
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
    /// let response = NpmController::search(&request);
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
    /// NpmController::search(&request)
    /// ```
    pub fn search(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let default = String::from(" ");
        let search_string = request.query_params().get("query").unwrap_or(&default);

        let mut weighted_matched: Vec<(usize, &str)> = NpmExpansions::expansions()
            .iter()
            .map(|expansion| (levenshtein(expansion, search_string), *expansion))
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
            _ => not_acceptable_response(),
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
    /// let response = NpmController::not_found(&request);
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
    /// NpmController::not_found(&request)
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
            _ => not_acceptable_response(),
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
    /// let response = NpmController::internal_server_error(&request);
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
    /// NpmController::internal_server_error(&request)
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
            _ => not_acceptable_response(),
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
    /// let response = NpmController::client_error(&request);
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
    /// NpmController::client_error(&request)
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
            _ => not_acceptable_response(),
        }
        .into_http_response();

        Ok(response)
    }
}

fn not_acceptable_response() -> Response {
    Response::new(
        "406 NOT ACCEPTABLE",
        "",
        "Please accept application/json".to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::all; "all")]
    #[test_case(NpmController::search; "search")]
    #[test_case(NpmController::not_found; "not_found")]
    #[test_case(NpmController::internal_server_error; "internal_server_error")]
    #[test_case(NpmController::client_error; "client_error")]

    fn valid_request(controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>) {
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        assert!(controller_function(&request).is_ok())
    }

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::all; "all")]
    #[test_case(NpmController::search; "search")]
    #[test_case(NpmController::not_found; "not_found")]
    #[test_case(NpmController::internal_server_error; "internal_server_error")]
    #[test_case(NpmController::client_error; "client_error")]
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

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::not_found; "not_found")]
    #[test_case(NpmController::internal_server_error; "internal_server_error")]
    #[test_case(NpmController::client_error; "client_error")]

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

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::not_found; "not_found")]
    #[test_case(NpmController::internal_server_error; "internal_server_error")]
    #[test_case(NpmController::client_error; "client_error")]

    fn no_accept_header(controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>) {
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());

        assert!(controller_function(&request).is_ok())
    }
}
