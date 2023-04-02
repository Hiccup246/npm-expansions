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
