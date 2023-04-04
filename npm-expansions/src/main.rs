use npm_expansions::{
    expansions_model::{ExpansionsAccess, ExpansionsModel},
    npm_controller::{ControllerFunction, NpmController},
    router::{Router, Routes},
    stream_handler,
};
use std::collections::HashMap;
use std::{env, net::TcpListener};

fn main() {
    let development_env = env::var("DEV").is_ok();
    let addr = if development_env {
        "0.0.0.0:8000"
    } else {
        "[::]:8080"
    };

    let expansions_generator =
        &ExpansionsModel::build("rsc/expansions.txt") as &dyn ExpansionsAccess;

    let config: Routes = HashMap::from([
        (
            "GET /api/random HTTP/1.1",
            NpmController::random as ControllerFunction,
        ),
        (
            "GET /api/all HTTP/1.1",
            NpmController::all as ControllerFunction,
        ),
        (
            "GET /api/search HTTP/1.1",
            NpmController::search as ControllerFunction,
        ),
    ]);

    let router = Router::new(config);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        if let Err(err) =
            stream_handler::handle_connection(&mut stream, &router, expansions_generator)
        {
            println!("Fatal server error. Error Message: {}", err)
        }
    }
}
