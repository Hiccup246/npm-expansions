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
        
        println!("Injecting env variable: {}", key);
        
        env_variables.insert(key.trim().to_string(), val.to_string());
    }

    println!("Injecting env variables into pages directory...");
    env_injector::inject_envs_into_drectory(
        PathBuf::from("pages"),
        PathBuf::from("injected_pages"),
        &env_variables,
    )
    .unwrap();
    println!("Successfully injected env variables into pages directory!");

    println!("Injecting env variables into static directory...");
    env_injector::inject_envs_into_drectory(
        PathBuf::from("static"),
        PathBuf::from("injected_static"),
        &env_variables,
    )
    .unwrap();
    println!("Successfully injected env variables into pages directory!");

    println!("Minifying pages directory...");
    minify::minify_drectory(
        PathBuf::from("injected_pages"),
        PathBuf::from("minified_pages"),
    )
    .unwrap();
    println!("Successfully minified pages directory!");

    println!("Minifying pages directory...");
    minify::minify_drectory(
        PathBuf::from("injected_static"),
        PathBuf::from("minified_static"),
    )
    .unwrap();
    println!("Successfully minified static directory!");
}
