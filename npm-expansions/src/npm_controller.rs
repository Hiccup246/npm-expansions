use crate::expansions_model::ExpansionsAccess;
use crate::mime_type::matcher;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::request::Request;
use crate::response::Response;

pub struct NpmController {}

pub type ControllerFunction =
    fn(&Request, &dyn ExpansionsAccess) -> Result<Vec<u8>, NpmExpansionsError>;

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
    /// use npm_expansions::{
    ///     npm_controller::NpmController,
    ///     request::Request,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_generator = &MockExpansionsModel::default();
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::new());
    /// let response = NpmController::random(&request, mock_generator);
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
    ///     request::Request,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_generator = &MockExpansionsModel::default();
    /// let request = Request::new("GET /random HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    ///
    /// NpmController::random(&request, mock_generator);
    /// ```
    pub fn random(
        request: &Request,
        expansions_model: &dyn ExpansionsAccess,
    ) -> Result<Vec<u8>, NpmExpansionsError> {
        let headers = request.headers();
        let accept_header = headers.get("Accept").or_else(|| headers.get("accept"));
        let best = matcher::best_match(
            Vec::from(["application/json"]),
            accept_header.unwrap_or(&"".to_string()),
        )?;

        let response = match best.as_str() {
            "application/json" => Response::new(
                "200 OK",
                "Content-Type: application/json",
                &format!(
                    "{{\"npm-expansion\": \"{}\"}}",
                    expansions_model.random_expansion()
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
    /// use npm_expansions::{
    ///     npm_controller::NpmController,
    ///     request::Request,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    //
    /// let mock_generator = &MockExpansionsModel::default();
    /// let request = Request::new("GET /all HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::new());
    /// let response = NpmController::all(&request, mock_generator);
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
    ///     request::Request,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_generator = &MockExpansionsModel::default();
    /// let request = Request::new("GET /all HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    ///
    /// NpmController::all(&request, mock_generator);
    /// ```
    pub fn all(
        request: &Request,
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
            .map(|expansions| format!("\"{expansions}\""))
            .collect();

        let response = match best.as_str() {
            "application/json" => Response::new(
                "200 OK",
                "Content-Type: application/json",
                &format!("[{}]", string_expansions.join(",")),
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
    /// use npm_expansions::{
    ///     npm_controller::NpmController,
    ///     request::Request,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     expansions_model::ExpansionsAccess,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let mock_generator = &MockExpansionsModel::default();
    /// let request = Request::new("GET /search HTTP/1.1", HashMap::from([("Accept".to_string(), "application/json".to_string())]), HashMap::from([("query".to_string(), "abc".to_string())]));
    /// let response = NpmController::search(&request, mock_generator);
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
    /// use npm_expansions::request::Request;
    /// use std::collections::HashMap;
    /// use npm_expansions::mock_expansions_model::MockExpansionsModel;
    /// use npm_expansions::expansions_model::ExpansionsAccess;
    /// let request = Request::new("GET /search HTTP/1.1", HashMap::from([("Accept".to_string(), "text/".to_string())]), HashMap::new());
    /// let mock_generator = &MockExpansionsModel::default();
    /// NpmController::search(&request, mock_generator);
    /// ```
    pub fn search(
        request: &Request,
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
        let top_ten: Vec<String> = expansions_model.search(search_string);

        let response = match best.as_str() {
            "application/json" => Response::new(
                "200 OK",
                "Content-Type: application/json",
                &format!("[{}]", top_ten.join(",")),
            ),
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
    use crate::mock_expansions_model::MockExpansionsModel;
    use std::collections::HashMap;
    use test_case::test_case;

    #[test_case(NpmController::random; "random")]
    #[test_case(NpmController::all; "all")]
    #[test_case(NpmController::search; "search")]

    fn valid_request(controller_function: ControllerFunction) {
        let request = Request::new(
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
        let request = Request::new(
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
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("Accept".to_string(), "text/".to_string())]),
            HashMap::new(),
        );

        let expansions_generator = &MockExpansionsModel::default();
        assert!(controller_function(&request, expansions_generator).is_err())
    }

    #[test_case(NpmController::random; "random")]

    fn lower_case_accept_header(controller_function: ControllerFunction) {
        let request = Request::new(
            "GET / HTTP/1.1",
            HashMap::from([("accept".to_string(), "text/html".to_string())]),
            HashMap::new(),
        );

        let expansions_generator = &MockExpansionsModel::default();
        assert!(controller_function(&request, expansions_generator).is_ok())
    }

    #[test_case(NpmController::random; "random")]

    fn no_accept_header(controller_function: ControllerFunction) {
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());

        let expansions_generator = &MockExpansionsModel::default();
        assert!(controller_function(&request, expansions_generator).is_ok())
    }
}
