use npm_expansions::expansions_updater;
use npm_expansions::history_model::HistoryModel;
use npm_expansions::{
    expansions_model::ExpansionsModel,
    npm_controller::{ControllerFunction, NpmController},
    router::Router,
    stream_handler,
    thread_pool::ThreadPool,
};
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::{collections::HashMap, env, net::TcpListener, path::Path, thread};

static EXPANSIONS_MODEL: Lazy<Arc<RwLock<ExpansionsModel>>> = Lazy::new(|| {
    Arc::new(RwLock::new(ExpansionsModel::new(Path::new(
        "rsc/expansions.txt",
    ))))
});

static HISTORY_MODEL: Lazy<Arc<RwLock<HistoryModel>>> =
    Lazy::new(|| Arc::new(RwLock::new(HistoryModel::new(Path::new("rsc/history.txt")))));

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
static NPM_EXPANSIONS_REPO_URL: &str = "https://api.github.com/repos/npm/npm-expansions";
static TWO_WEEKS_IN_MILLIS: i64 = 60000 * 60 * 24 * 14;

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

    let update_interval = match env::var("UPDATER_INTERVAL") {
        Ok(interval) => interval.parse::<i64>().unwrap_or(TWO_WEEKS_IN_MILLIS),
        Err(_err) => TWO_WEEKS_IN_MILLIS,
    };

    thread::Builder::new()
        .spawn(move || loop {
            let mut updater = expansions_updater::ExpansionsUpater::new(
                EXPANSIONS_MODEL.clone(),
                HISTORY_MODEL.clone(),
                NPM_EXPANSIONS_REPO_URL.to_string(),
                update_interval
            );

            let next_update_in = updater.time_to_next_update(chrono::Utc::now()).unwrap_or_else(|err|{
                print!("Updater failed to calculate next interval time with error: {}...defaulting to {}", err, TWO_WEEKS_IN_MILLIS);
                Duration::from_millis(TWO_WEEKS_IN_MILLIS as u64)
            });

            thread::sleep(next_update_in);

            match updater.add_new_expansions() {
                Ok(res) => println!("Updater successfully added new expansions! {:?}", res),
                Err(err) => println!("Updater failed to add new expansion with error: {}", err)
            }
        }).expect("Failed to spawn background thread to update expansions model");

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
