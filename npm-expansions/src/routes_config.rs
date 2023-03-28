use crate::request::Request;
use crate::router::Route;
use crate::Controller;
use crate::NpmExpansionsError;
use std::collections::HashMap;

/// Returns the http routes supported by this projects server
///
/// # Arguments
///
/// * `static_directory` - A string representing the root directory where static to be served
/// by the server e.g. images are located
///
/// # Examples
///
/// ```
/// let config = route_config();
/// assert!(config);
/// ```
///
/// # Panics
///
/// The function panics if the given static_directory does not exist or if it
/// contains files whoose names cannot become strings
///
pub fn route_config() -> Route {
    let config: Route = HashMap::from([
        (
            "GET /api/random HTTP/1.1".to_string(),
            Controller::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
        (
            "GET /api/all HTTP/1.1".to_string(),
            Controller::all as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
        (
            "GET /api/search HTTP/1.1".to_string(),
            Controller::search as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
        (
            "404".to_string(),
            Controller::not_found as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
    ]);

    config
}
