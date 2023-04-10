use asset_compiler::env_injector;
use asset_compiler::minify;
use std::{collections::HashMap, env, path::PathBuf};

fn main() {
    let mut env_variables: HashMap<String, String> = HashMap::new();
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args {
        let (key, val) = arg
            .split_once('=')
            .expect("Command line arguments must be in format KEY=val");
        env_variables.insert(key.trim().to_string(), val.to_string());
    }

    env_injector::inject_envs_into_drectory(
        PathBuf::from("pages"),
        PathBuf::from("injected_pages"),
        &env_variables,
    )
    .unwrap();
    env_injector::inject_envs_into_drectory(
        PathBuf::from("static"),
        PathBuf::from("injected_static"),
        &env_variables,
    )
    .unwrap();

    minify::minify_drectory(
        PathBuf::from("injected_pages"),
        PathBuf::from("minified_pages"),
    )
    .unwrap();
    minify::minify_drectory(
        PathBuf::from("injected_static"),
        PathBuf::from("minified_static"),
    )
    .unwrap();
}
