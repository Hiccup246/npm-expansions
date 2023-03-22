use crate::request::Request;
use crate::Controller;
use std::{collections::HashMap, fs};

pub fn route_config() -> HashMap<String, fn(&Request) -> Vec<u8>> {
    let mut config: HashMap<String, fn(&Request) -> Vec<u8>> = HashMap::from([
        (
            "GET / HTTP/1.1".to_string(),
            Controller::index as fn(&Request) -> Vec<u8>,
        ),
        (
            "GET /random HTTP/1.1".to_string(),
            Controller::random as fn(&Request) -> Vec<u8>,
        ),
        (
            "404".to_string(),
            Controller::not_found as fn(&Request) -> Vec<u8>,
        ),
    ]);

    insert_static_routes(&mut config);

    config
}

fn insert_static_routes(routes_config: &mut HashMap<String, fn(&Request) -> Vec<u8>>) {
    let static_file_names: Vec<String> = directory_file_names("static".to_string())
        .iter()
        .map(|file_name| format!("GET /{file_name} HTTP/1.1"))
        .collect();

    for static_file in static_file_names {
        routes_config.insert(
            static_file,
            Controller::static_file as fn(&Request) -> Vec<u8>,
        );
    }
}

fn directory_file_names(directory_path: String) -> Vec<String> {
    let directory = fs::read_dir(directory_path).unwrap();
    directory
        .map(|dir_entry| dir_entry.unwrap())
        .filter(|dir_entry| dir_entry.file_type().unwrap().is_file())
        .map(|dir_entry| dir_entry.file_name().into_string().unwrap())
        .collect()
}
