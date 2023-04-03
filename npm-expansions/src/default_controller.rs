use crate::accept_header_handler;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::response::Response;
use crate::router::Request;

pub struct DefaultController {}

impl DefaultController {
    /// Returns a vector byte representation of the not_found page including html, css and javascript
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// use npm_expansions::{
    ///     default_controller::DefaultController,
    ///     request::Request
    /// };
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::new());
    /// let response = DefaultController::not_found(&request);
    ///
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// use npm_expansions::{
    ///     default_controller::DefaultController,
    ///     request::Request,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    ///
    /// DefaultController::not_found(&request);
    /// ```
    pub fn not_found(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => Response::new("404 NOT FOUND", "", "NOT FOUND"),
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
    /// use npm_expansions::{
    ///     default_controller::DefaultController,
    ///     request::Request,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::new());
    /// let response = DefaultController::internal_server_error(&request);
    ///
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// use npm_expansions::{
    ///     default_controller::DefaultController,
    ///     request::Request,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new("GET / HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    ///
    /// DefaultController::internal_server_error(&request);
    /// ```
    pub fn internal_server_error(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => {
                Response::new("500 INTERNAL SERVER ERROR", "", "INTERNAL SERVER ERROR")
            }
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
    /// use npm_expansions::{
    ///     default_controller::DefaultController,
    ///     request::Request,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new("GET /non-existant/route HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::new());
    /// let response = DefaultController::client_error(&request);
    ///
    /// assert!(response.is_ok());
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if the given request has invalid headers
    ///
    /// ```rust,should_error
    /// // fails if the given request has invalid headers
    /// use npm_expansions::{
    ///     default_controller::DefaultController,
    ///     request::Request,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let request = Request::new("GET / HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    ///
    /// DefaultController::client_error(&request);
    /// ```
    pub fn client_error(request: &Request) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = accept_header_handler::best_match(
            Vec::from(["application/json", "text/html"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => Response::new("400 BAD REQUEST", "", "BAD REQUEST"),
            _ => not_acceptable_response(),
        }
        .into_http_response();

        Ok(response)
    }
}

fn not_acceptable_response() -> Response {
    Response::new("406 NOT ACCEPTABLE", "", "Please accept application/json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(DefaultController::not_found; "not_found")]
    #[test_case(DefaultController::internal_server_error; "internal_server_error")]
    #[test_case(DefaultController::client_error; "client_error")]
    fn valid_request(controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>) {
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        assert!(controller_function(&request).is_ok())
    }

    #[test_case(DefaultController::not_found; "not_found")]
    #[test_case(DefaultController::internal_server_error; "internal_server_error")]
    #[test_case(DefaultController::client_error; "client_error")]

    fn valid_request_returns_content(
        controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
    ) {
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        assert!(controller_function(&request).unwrap().len() > 0)
    }

    #[test_case(DefaultController::not_found; "not_found")]
    #[test_case(DefaultController::internal_server_error; "internal_server_error")]
    #[test_case(DefaultController::client_error; "client_error")]
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

    #[test_case(DefaultController::not_found; "not_found")]
    #[test_case(DefaultController::internal_server_error; "internal_server_error")]
    #[test_case(DefaultController::client_error; "client_error")]

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

    #[test_case(DefaultController::not_found; "not_found")]
    #[test_case(DefaultController::internal_server_error; "internal_server_error")]
    #[test_case(DefaultController::client_error; "client_error")]

    fn no_accept_header(controller_function: fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>) {
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());

        assert!(controller_function(&request).is_ok())
    }
}
