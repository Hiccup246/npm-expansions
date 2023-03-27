use crate::request::Request;
use crate::Controller;
use crate::NpmExpansionsError;
use std::{collections::HashMap, fs};
use crate::router::Route;

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
pub fn route_config(
    static_directory: &str,
) -> Route {
    let mut config: Route =
        HashMap::from([
            (
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            ),
            (
                "GET /random HTTP/1.1".to_string(),
                Controller::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            ),
            (
                "404".to_string(),
                Controller::not_found as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            ),
        ]);

    insert_static_routes(
        &mut config,
        generate_static_file_route_names(static_directory),
    );

    config
}

fn insert_static_routes(
    routes_config: &mut Route,
    static_file_route_names: Vec<String>,
) {
    for static_file in static_file_route_names {
        routes_config.insert(
            static_file,
            Controller::static_file as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        );
    }
}

fn generate_static_file_route_names(directory_path: &str) -> Vec<String> {
    directory_file_names(directory_path.to_string())
        .iter()
        .map(|file_name| format!("GET /{file_name} HTTP/1.1"))
        .collect()
}

fn directory_file_names(directory_path: String) -> Vec<String> {
    let directory = fs::read_dir(directory_path).unwrap();
    directory
        .map(|dir_entry| dir_entry.unwrap())
        .filter(|dir_entry| dir_entry.file_type().unwrap().is_file())
        .map(|dir_entry| dir_entry.file_name().into_string().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod route_config {
        extern crate tempdir;

        use super::*;
        use tempdir::TempDir;

        #[test]
        fn valid_static_directory() {
            let dir = TempDir::new("my_directory_prefix").unwrap();
            let _joined_path = dir.path().join("foo.txt");

            let config = route_config(dir.path().to_str().unwrap());
            let mut config_keys: Vec<&String> = config.keys().collect();

            let expected_hash_map = HashMap::from([
                (
                    "GET / HTTP/1.1".to_string(),
                    Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
                ),
                (
                    "GET /random HTTP/1.1".to_string(),
                    Controller::random as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
                ),
                (
                    "404".to_string(),
                    Controller::not_found as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
                ),
                (
                    "GET /foo HTTP/1.1".to_string(),
                    Controller::static_file as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
                ),
            ]);
            let mut expected_hash_map_keys: Vec<&String> = expected_hash_map.keys().collect();

            assert_eq!(config_keys.sort(), expected_hash_map_keys.sort());
        }

        #[test]
        #[should_panic(expected = "No such file or directory")]
        fn invalid_static_directory() {
            route_config("non-existant-directory-path");
        }
    }
}
