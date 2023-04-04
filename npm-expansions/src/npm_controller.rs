use crate::expansions_model::ExpansionsAccess;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::mime_type::matcher;
use crate::npm_expansion_error::NpmExpansionsError;

/// A collection of functions which accept a request and expansions model and use
/// them to produce JSON responses
pub struct NpmController {}

/// The function signature of NpmController functions
pub type ControllerFunction =
    fn(&HttpRequest, &dyn ExpansionsAccess) -> Result<Vec<u8>, NpmExpansionsError>;

impl NpmController {
    /// Returns a vector byte representation of a json object containing a random npm expansion.
    /// ```json
    /// { "npm-expansion": "Nice Pistons Mac" }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// use npm_expansions::{
    ///     npm_controller::NpmController,
    ///     http_request::HttpRequest,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_expansions_model = &MockExpansionsModel::default();
    /// let request = HttpRequest::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::new());
    /// let response = NpmController::random(&request, mock_expansions_model);
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
    ///     npm_controller::NpmController,
    ///     http_request::HttpRequest,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_expansions_model = &MockExpansionsModel::default();
    /// let request = HttpRequest::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    ///
    /// NpmController::random(&request, mock_expansions_model);
    /// ```
    pub fn random(
        request: &HttpRequest,
        expansions_model: &dyn ExpansionsAccess,
    ) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = matcher::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => HttpResponse::new(
                "200 OK",
                "Content-Type: application/json",
                &format!(
                    "{{\"npm-expansion\": \"{}\"}}",
                    expansions_model.random_expansion()
                ),
            ),
            _ => not_acceptable_response(),
        }
        .into_bytes_vec();

        Ok(response)
    }

    /// Returns a vector byte representation of a json array containing all npm expansions
    /// ```json
    /// ["Nice Pistons Mac", "Nicole Pasta Mcdougle"]
    /// ```
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// use npm_expansions::{
    ///     npm_controller::NpmController,
    ///     http_request::HttpRequest,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    //
    /// let mock_expansions_model = &MockExpansionsModel::default();
    /// let request = HttpRequest::new("GET /all HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::new());
    /// let response = NpmController::all(&request, mock_expansions_model);
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
    ///     npm_controller::NpmController,
    ///     http_request::HttpRequest,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_expansions_model = &MockExpansionsModel::default();
    /// let request = HttpRequest::new("GET /all HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    ///
    /// NpmController::all(&request, mock_expansions_model);
    /// ```
    pub fn all(
        request: &HttpRequest,
        expansions_model: &dyn ExpansionsAccess,
    ) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = matcher::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let string_expansions: Vec<String> = expansions_model
            .all()
            .iter()
            .map(|expansion| format!("\"{expansion}\""))
            .collect();

        let response = match best.as_str() {
            "application/json" => HttpResponse::new(
                "200 OK",
                "Content-Type: application/json",
                &format!("[{}]", string_expansions.join(",")),
            ),
            _ => not_acceptable_response(),
        }
        .into_bytes_vec();

        Ok(response)
    }

    /// Returns a vector byte representation of a json array containing the top ten matches of npm expansions given
    /// a request with a search_query field
    /// ```json
    /// ["Nice Pistons Mac", "Nicole Pasta Mcdougle"]
    /// ```
    ///
    /// # Arguments
    ///
    /// * `request` - An incoming HTTP request
    ///
    /// # Examples
    ///
    /// ```
    /// use npm_expansions::{
    ///     npm_controller::NpmController,
    ///     http_request::HttpRequest,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_expansions_model = &MockExpansionsModel::default();
    /// let request = HttpRequest::new("GET /search HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::from([("query".to_string(), "abc".to_string())]));
    /// let response = NpmController::search(&request, mock_expansions_model);
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
    /// use npm_expansions::npm_controller::NpmController;
    /// use npm_expansions::http_request::HttpRequest;
    /// use std::collections::HashMap;
    /// use npm_expansions::mock_expansions_model::MockExpansionsModel;
    /// use npm_expansions::expansions_model::ExpansionsAccess;
    /// let request = HttpRequest::new("GET /search HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    /// let mock_expansions_model = &MockExpansionsModel::default();
    /// NpmController::search(&request, mock_expansions_model);
    /// ```
    pub fn search(
        request: &HttpRequest,
        expansions_model: &dyn ExpansionsAccess,
    ) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = matcher::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let default = String::from(" ");
        let search_string = request.query_params().get("query").unwrap_or(&default);
        let top_ten: Vec<String> = expansions_model
            .search(search_string)
            .iter()
            .map(|expansion| format!("\"{expansion}\""))
            .collect();

        let response = match best.as_str() {
            "application/json" => HttpResponse::new(
                "200 OK",
                "Content-Type: application/json",
                &format!("[{}]", top_ten.join(",")),
            ),
            _ => not_acceptable_response(),
        }
        .into_bytes_vec();

        Ok(response)
    }
}

fn not_acceptable_response() -> HttpResponse {
    HttpResponse::new("406 NOT ACCEPTABLE", "", "Please accept application/json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_expansions_model::MockExpansionsModel;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::all; "all")]
    #[test_case(NpmController::search; "search")]

    fn valid_request(controller_function: ControllerFunction) {
        let request = HttpRequest::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        let expansions_generator = &MockExpansionsModel::default();
        assert!(controller_function(&request, expansions_generator).is_ok())
    }

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::all; "all")]
    #[test_case(NpmController::search; "search")]
    fn valid_request_returns_content(controller_function: ControllerFunction) {
        let request = HttpRequest::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        let expansions_generator = &MockExpansionsModel::default();
        assert!(
            controller_function(&request, expansions_generator)
                .unwrap()
                .len()
                > 0
        )
    }

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::all; "all")]
    #[test_case(NpmController::search; "search")]
    fn invalid_request_headers(controller_function: ControllerFunction) {
        let request = HttpRequest::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/".to_string())]),
            HashMap::new(),
        );

        let expansions_generator = &MockExpansionsModel::default();
        assert!(controller_function(&request, expansions_generator).is_err())
    }

    #[test_case(NpmController::random; "random")]

    fn lower_case_accept_header(controller_function: ControllerFunction) {
        let request = HttpRequest::new(
            "GET / HTTP/1.1",
            HashMap::from([("accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        let expansions_generator = &MockExpansionsModel::default();
        assert!(controller_function(&request, expansions_generator).is_ok())
    }

    #[test_case(NpmController::random; "random")]

    fn no_accept_header(controller_function: ControllerFunction) {
        let request = HttpRequest::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());

        let expansions_generator = &MockExpansionsModel::default();
        assert!(controller_function(&request, expansions_generator).is_ok())
    }
}
