use npm_expansions::router;
use npm_expansions::routes_config;
use npm_expansions::stream_handler;
use std::{env, net::TcpListener};

fn main() {
    let development_env = env::var("DEV").is_ok();
    let addr = if development_env {
        "0.0.0.0:8000"
    } else {
        "[::]:8080"
    };

    let router = router::Router::new(routes_config::route_config());
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        stream_handler::handle_connection(&mut stream, &router);
    }
}

// The library i am creating should allow users i.e. developers to:
// Define a route config which references ther own controllers which use a trait
// i have defined.
// My library should provide errors and mock structs for testing
// My library should expose the response struct but not the request one

// Things I think should be public - based on this being a basic library framework
// Route config type
// Router init
// stream handler
// controller trait
// response struct
// mock tcp stream
// npm expansions error
// mime type best match function

// I could choose to make this a complete utility framework where everything is public
