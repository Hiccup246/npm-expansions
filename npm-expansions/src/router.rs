use crate::default_controller::DefaultController;
use crate::expansions_model::ExpansionsAccess;
use crate::npm_controller::ControllerFunction;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::request::Request;
use std::collections::HashMap;

pub type Route = HashMap<&'static str, ControllerFunction>;

#[derive(Debug)]
pub struct HandleRouteError;

pub struct Router {
    routes_config: Route,
}

impl Router {
    pub fn new(routes_config: Route) -> Router {
        Router { routes_config }
    }

    /// Returns a byte response to an incoming request
    ///
    /// # Arguments
    ///
    /// * `request` - A Request struct
    ///
    /// # Examples
    ///
    /// ```
    /// use npm_expansions::{
    ///     router::{Router, Route},
    ///     request::Request,
    ///     mock_expansions_model::MockExpansionsModel,
    ///     npm_expansion_error::NpmExpansionsError,
    ///     expansions_model::ExpansionsAccess,
    ///     npm_controller::ControllerFunction,
    /// };
    /// use std::collections::HashMap;
    ///
    /// let actual_route: ControllerFunction = |_,_| Ok("actual_route".as_bytes().to_vec());
    /// let route_config: Route =
    ///     HashMap::from([("GET / HTTP/1.1", actual_route)]);
    /// let router = Router::new(route_config);
    /// let request = Request::new("GET / HTTP/1.1", HashMap::new(),  HashMap::new());
    /// let mock_generator = &MockExpansionsModel::default();
    /// let response = router.route_request(request, mock_generator);
    ///
    /// assert_eq!(response.unwrap(), "actual_route".as_bytes().to_vec());
    /// ```
    pub fn route_request(
        &self,
        request: Request,
        expansions_generator: &dyn ExpansionsAccess,
    ) -> Result<Vec<u8>, NpmExpansionsError> {
        let status_line = request.status_line_stripped();
        let controller_function = self.routes_config.get(status_line.as_str());

        if let Some(controller_function) = controller_function {
            return controller_function(&request, expansions_generator);
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
        let actual_route: ControllerFunction = |_, _| Ok("actual_route".as_bytes().to_vec());
        let route_config: Route = HashMap::from([("GET / HTTP/1.1", actual_route)]);

        let router = Router::new(route_config);
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());
        let mock_generator = &MockExpansionsModel::default();
        let response = router.route_request(request, mock_generator);

        assert_eq!(response.unwrap(), "actual_route".as_bytes().to_vec())
    }

    #[test]
    fn route_not_found() {
        let not_found: ControllerFunction = |_, _| Ok("not_found".as_bytes().to_vec());
        let route_config: Route = HashMap::from([("404", not_found)]);
        let router = Router::new(route_config);
        let request = Request::new("GET /fake_route HTTP/1.1", HashMap::new(), HashMap::new());
        let mock_generator = &MockExpansionsModel::default();
        let response = router.route_request(request, mock_generator);

        assert!(response.is_ok())
    }

    #[test]
    fn no_route() {
        let router = Router::new(HashMap::new());
        let request = Request::new("GET / HTTP/1.1", HashMap::new(), HashMap::new());
        let mock_generator = &MockExpansionsModel::default();
        let response = router.route_request(request, mock_generator);

        assert!(response.is_ok())
    }
}
