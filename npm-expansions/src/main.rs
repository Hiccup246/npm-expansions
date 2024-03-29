use npm_expansions::{
    expansions_model::ExpansionsModel,
    npm_controller::{ControllerFunction, NpmController},
    router::Router,
    stream_handler,
    thread_pool::ThreadPool,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::{env, net::TcpListener};

static EXPANSIONS_MODEL: Lazy<Arc<RwLock<ExpansionsModel>>> =
    Lazy::new(|| Arc::new(RwLock::new(ExpansionsModel::build("rsc/expansions.txt"))));

static ROUTER: Lazy<Arc<Router>> = Lazy::new(|| {
    Arc::new(Router::new(HashMap::from([
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
    ])))
});

static DEFAULT_THREAD_COUNT: usize = 2;

fn main() {
    let thread_count = env::var("THREAD_COUNT")
        .map(|count| count.parse::<usize>().unwrap_or(DEFAULT_THREAD_COUNT))
        .unwrap_or(DEFAULT_THREAD_COUNT);

    println!(
        "Initializing server with a thread pool of: {}",
        thread_count
    );

    let development_env = env::var("DEV").is_ok();
    let addr = if development_env {
        "0.0.0.0:8000"
    } else {
        "[::]:8080"
    };

    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let pool = ThreadPool::new(thread_count);

        let execution_result = pool.execute(move || {
            stream_handler::handle_connection(
                &mut stream,
                ROUTER.clone(),
                EXPANSIONS_MODEL.clone(),
            )
            .unwrap_or_else(|error| println!("Fatal server error. Error Message: {}", error));
        });

        if let Err(execution_err) = execution_result {
            println!("Failed to execute clojure: {:?}", execution_err)
        }
    }
}
