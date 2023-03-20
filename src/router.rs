use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub use crate::request::Request;

pub struct Router<'a> {
    // Handle 404 routes
    // Handle incorrect version routes
    // Handle static routes
    // Handle legit routes

    // Start-up will include building static file routes
    // stream: &TcpStream,
    // static_files_directory: &str,
    routes_config: HashMap<&'a str,fn(&Request) -> Vec<u8>>,
}

// There will be two controllers
// - npm-expansions controller (no need to split 500 or 404 into separate 'default' controller)
// - static files controller

// We know ahead of time what all the routes will be
// Therefor we should pass a router a config file mapping
// route names to controller methods. And the router should merely call
// the router methods
impl Router<'_> {
    pub fn new<'a>(
        routes_config: HashMap<&'a str,fn(&Request) -> Vec<u8>>,
    ) -> Router<'a> {
        Router {
            routes_config,
        }
    }

    pub fn route_request(&self, request: Request) -> Vec<u8> {
        let status_line = request.status_line();
        let controller_function = self.routes_config.get(status_line);

        if let Some(controller_function) = controller_function {
            return controller_function(&request);
        } else {

        }
        // Build request headers here
        // Parse request so can route effectivley
        // match request to route config
        // call controller method
        // let controller_method = get_controller_method
        // No need for controller to know status line
        // let response = controller_method(request_headers, request_body)

        // self.stream.write(status_line)
        // self.stream.write(headers)
        // self.stream.write(content)

        // If error then 500
        // - Use npm-expansions controller
        // If no route then 404
        // - Use npm-expansions controller

        // QUESTION
        // Should the router know about the stream? Perhaps it should  just return a byte response
        // and then the main function just "writes out"
        "hello".as_bytes().to_vec()
    }
}

// Hypothetical main function

// let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
// let router = Router::new(stastic_files_directory, config);

// for stream in listener.incoming() {
//     let stream = stream.unwrap();
//
//     OPTION 1 - Build request in router and responsd to stream
//     router.route_request(stream);
//
//     OPTION 2 - Build request outside of router (assuming we can combine byte arrays)
//     let request = RequestBuilder::construct_request(stream);
//     let response = router.route_request(request);
//     stream.write_all(response);
// }
