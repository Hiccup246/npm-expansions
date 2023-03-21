use std::collections::HashMap;

pub use crate::request::Request;

#[derive(Debug)]
pub struct HandleRouteError;

pub struct Router<'a> {
    routes_config: HashMap<&'a str, fn(&Request) -> Vec<u8>>,
}

impl Router<'_> {
    pub fn new<'a>(routes_config: HashMap<&'a str, fn(&Request) -> Vec<u8>>) -> Router<'a> {
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
    /// let actual_route: fn(&Request) -> Vec<u8> = |_| "actual_route".as_bytes().to_vec();
    /// let route_config: HashMap<&str, fn(&Request) -> Vec<u8>> =
    ///     HashMap::from([("GET / HTTP/1.1", actual_route)]);
    /// let router = Router::new(route_config);
    /// let request = Request {
    ///     status_line: "GET / HTTP/1.1".to_string(),
    ///     headers: HashMap::new(),
    /// };
    /// let response = router.route_request(request);
    /// assert_eq!(response.unwrap(), "actual_route".as_bytes().to_vec())
    /// ```
    ///
    /// # Failures
    ///
    /// The function fails if an unrecognized route is given and no 404 route exists
    ///
    /// ```rust,should_error
    /// // fails if no 404 route exists and a unknown route is given
    /// let router = Router::new(HashMap::new());
    /// let request = Request {
    ///     status_line: "GET / HTTP/1.1".to_string(),
    ///     headers: HashMap::new(),
    /// };
    /// let response = router.route_request(request);
    /// assert!(response.is_err())
    /// ```
    pub fn route_request(&self, request: Request) -> Result<Vec<u8>, HandleRouteError> {
        let status_line = request.status_line();
        let controller_function = self.routes_config.get(status_line);

        if let Some(controller_function) = controller_function {
            return Ok(controller_function(&request));
        }

        if let Some(not_found_route) = self.routes_config.get("404") {
            return Ok(not_found_route(&request));
        } else {
            Err(HandleRouteError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_response() {
        let actual_route: fn(&Request) -> Vec<u8> = |_| "actual_route".as_bytes().to_vec();
        let route_config: HashMap<&str, fn(&Request) -> Vec<u8>> =
            HashMap::from([("GET / HTTP/1.1", actual_route)]);
        let router = Router::new(route_config);
        let request = Request::new("GET / HTTP/1.1", HashMap::new());
        let response = router.route_request(request);

        assert_eq!(response.unwrap(), "actual_route".as_bytes().to_vec())
    }

    #[test]
    fn route_not_found() {
        let not_found: fn(&Request) -> Vec<u8> = |_| "not_found".as_bytes().to_vec();
        let route_config: HashMap<&str, fn(&Request) -> Vec<u8>> =
            HashMap::from([("404", not_found)]);
        let router = Router::new(route_config);
        let request = Request::new("GET /fake_route HTTP/1.1", HashMap::new());
        let response = router.route_request(request);

        assert_eq!(response.unwrap(), "not_found".as_bytes().to_vec())
    }

    #[test]
    fn no_route() {
        let router = Router::new(HashMap::new());
        let request = Request::new("GET / HTTP/1.1", HashMap::new());
        let response = router.route_request(request);

        assert!(response.is_err())
    }
}
