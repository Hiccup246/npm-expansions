use crate::default_controller::DefaultController;
use crate::expansions_model::ExpansionsAccess;
use crate::npm_controller::ControllerFunction;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::request::Request;
use std::collections::HashMap;

/// A type representing a series of http routes and their associated controller functions
/// # Examples
///
/// ```
/// # use crate::npm_expansions::npm_controller::{NpmController, ControllerFunction};
/// # use std::collections::HashMap;
/// HashMap::from([
///     (
///         "GET / HTTP/1.1",
///         NpmController::random as ControllerFunction,
///     )
/// ]);
/// ```
pub type Routes = HashMap<&'static str, ControllerFunction>;

/// A struct which stores a route
pub struct Router {
    routes_config: Routes,
}

impl Router {
    /// Creates a new router given a RouteConfig
    pub fn new(routes_config: Routes) -> Router {
        Router { routes_config }
    }

    /// Returns a byte response to an incoming request by matching the requests status line
    /// to its own routes config field
    ///
    /// # Arguments
    ///
    /// * `request` - A Request struct
    ///
    /// # Examples
    ///
    /// ```
    /// # use npm_expansions::{
    /// #    router::{Routes, Router},
    /// #    request::Request,
    /// #    mock_expansions_model::MockExpansionsModel,
    /// #    npm_expansion_error::NpmExpansionsError,
    /// #    expansions_model::ExpansionsAccess,
    /// #    npm_controller::ControllerFunction,
    /// # };
    /// # use std::collections::HashMap;
    ///
    /// # let actual_route: ControllerFunction = |_,_| Ok("actual_route".as_bytes().to_vec());
    /// # let route_config: Routes =
    /// #     HashMap::from([("GET / HTTP/1.1", actual_route)]);
    /// let router = Router::new(route_config);
    /// let request = Request::new("GET / HTTP/1.1", HashMap::new(),  HashMap::new());
    /// let mock_expansions_model = &MockExpansionsModel::default();
    /// let response = router.route_request(request, mock_expansions_model);
    ///
    /// assert_eq!(response.unwrap(), "actual_route".as_bytes().to_vec());
    /// ```
    pub fn route_request(
        &self,
        request: Request,
        expansions_model: &dyn ExpansionsAccess,
    ) -> Result<Vec<u8>, NpmExpansionsError> {
        let status_line = request.status_line_stripped();
        let controller_function = self.routes_config.get(status_line.as_str());

        if let Some(controller_function) = controller_function {
            return controller_function(&request, expansions_model);
        }

        DefaultController::not_found(&request)
    }
}

#[cfg(test)]
mod tests {
    use crate::mock_expansions_model::MockExpansionsModel;

    use super::*;

    #[test]
    fn route_response() {
        let controller_function: ControllerFunction = |_, _| Ok("actual_route".as_bytes().to_vec());
        let route_config: Routes = HashMap::from([("GET / HTTP/1.1", controller_function)]);

        let router = Router::new(route_config);
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());
        let mock_expansions_model = &MockExpansionsModel::default();
        let response = router.route_request(request, mock_expansions_model);

        assert_eq!(response.unwrap(), "actual_route".as_bytes().to_vec())
    }

    #[test]
    fn route_not_found() {
        let not_found: ControllerFunction = |_, _| Ok("not_found".as_bytes().to_vec());
        let route_config: Routes = HashMap::from([("404", not_found)]);

        let router = Router::new(route_config);
        let request = Request::new("GET /fake_route HTTP/1.1", HashMap::new(), HashMap::new());
        let mock_expansions_model = &MockExpansionsModel::default();
        let response = router.route_request(request, mock_expansions_model);

        assert!(response.is_ok())
    }

    #[test]
    fn no_route() {
        let router = Router::new(HashMap::new());
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());
        let mock_expansions_model = &MockExpansionsModel::default();
        let response = router.route_request(request, mock_expansions_model);

        assert!(response.is_ok())
    }
}
