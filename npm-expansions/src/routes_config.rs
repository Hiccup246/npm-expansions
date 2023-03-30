use crate::npm_controller::NpmController;
use crate::npm_expansion_error::NpmExpansionsError;
use crate::request::Request;
use crate::router::Route;
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
/// use npm_expansions::routes_config::route_config;
/// let config = route_config();
/// assert!(config.len() > 0);
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
            NpmController::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
        (
            "GET /api/all HTTP/1.1".to_string(),
            NpmController::all as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
        (
            "GET /api/search HTTP/1.1".to_string(),
            NpmController::search as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
        (
            "404".to_string(),
            NpmController::not_found as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        ),
    ]);

    config
}
