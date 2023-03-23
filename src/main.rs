use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod accept_header_handler;
mod controller;
mod mime_type_parser;
mod mock_tcp_stream;
mod npm_expansion_error;
mod npm_expansions;
mod request;
mod response;
mod router;
mod routes_config;

pub use crate::npm_expansions::NpmExpansions;
pub use controller::Controller;
use npm_expansion_error::{NpmErrorKind, NpmExpansionsError};
pub use request::Request;

fn main() {
    let router = router::Router::new(routes_config::route_config("static"));
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_connection(&mut stream, &router);
    }
}

fn handle_connection(stream: &mut (impl Read + Write), router: &router::Router) {
    let response = respond_to_request(stream, router);

    if let Err(res) = response {
        let error_request = Request::new(
            "",
            HashMap::from([(
                "Accept".to_string(),
                "text/html,application/json".to_string(),
            )]),
        );

        let error_response = match res.kind() {
            NpmErrorKind::InvalidHeader => Controller::client_error(&error_request),
            NpmErrorKind::TooManyHeaders => Controller::client_error(&error_request),
            NpmErrorKind::InvalidRequestStatusLine => Controller::client_error(&error_request),
            NpmErrorKind::InternalServerError => Controller::internal_server_error(&error_request),
            NpmErrorKind::RequestParseError => Controller::internal_server_error(&error_request),
            NpmErrorKind::SupportedMimeTypeError => {
                Controller::internal_server_error(&error_request)
            }
        };

        if let Ok(error_res) = error_response {
            return stream.write_all(error_res.as_slice()).unwrap();
        }

        panic!("Unconditional server failure. This is a server bug!")
    }
}

fn respond_to_request(
    stream: &mut (impl Read + Write),
    router: &router::Router,
) -> Result<(), NpmExpansionsError> {
    let request = Request::build(stream)?;
    let response = router.route_request(request)?;

    stream.write_all(response.as_slice()).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use super::*;
    use crate::mock_tcp_stream::MockTcpStream;

    mod respond_to_request {
        use super::*;

        #[test]
        fn valid_http_stream() {
            let input_bytes = b"GET / HTTP/1.1\r\nAccept: text/html\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = respond_to_request(&mut stream, &router);

            assert!(response.is_ok());
        }

        #[test]
        fn no_not_found_route() {
            let input_bytes = b"GET /non-existant/route HTTP/1.1\r\nAccept: text/html\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = respond_to_request(&mut stream, &router);

            assert!(response.is_err());
        }

        #[test]
        fn invalid_request() {
            let input_bytes = b"";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = respond_to_request(&mut stream, &router);

            assert!(response.is_err());
        }
    }

    mod connection_handler {
        use super::*;

        #[test]
        fn valid_http_stream() {
            let input_bytes = b"GET / HTTP/1.1\r\nAccept: text/html\r\n\r\n";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = handle_connection(&mut stream, &router);

            assert_eq!(response, ());
        }

        #[test]
        fn invalid_http_stream() {
            let input_bytes = b"";
            let mut contents = vec![0u8; 1024];

            contents[..input_bytes.len()].clone_from_slice(input_bytes);

            let mut stream = MockTcpStream {
                read_data: contents,
                write_data: Vec::new(),
            };

            let router = router::Router::new(HashMap::from([(
                "GET / HTTP/1.1".to_string(),
                Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
            )]));

            let response = handle_connection(&mut stream, &router);

            assert_eq!(response, ());
        }

        // #[test]
        // fn invalid_http_stream_with_error_response() {
        //     let input_bytes = b"";
        //     let mut contents = vec![0u8; 1024];

        //     contents[..input_bytes.len()].clone_from_slice(input_bytes);

        //     let mut stream = MockTcpStream {
        //         read_data: contents,
        //         write_data: Vec::new(),
        //     };

        //     fn panic_controller_method(request: &Request) -> Result<Vec<u8>, NpmExpansionsError>{

        //     }

        //     let router = router::Router::new(
        //         HashMap::from([
        //             (
        //                 "GET / HTTP/1.1".to_string(),
        //                 Controller::index as fn(&Request) -> Result<Vec<u8>, NpmExpansionsError>,
        //             )
        //         ])
        //     );

        //     let response = handle_connection(&mut stream, &router);

        //     assert_eq!(response, ());
        // }
    }
    // fn setup_server_and_connection() -> (TcpListener, TcpStream) {
    //     (
    //         TcpListener::bind("127.0.0.1:7878").unwrap(),
    //         TcpStream::connect("127.0.0.1:7878").unwrap(),
    //     )
    // }

    // #[test]
    // fn handle_connection_get_root() {
    //     let (mut listener, mut connection) = setup_server_and_connection();
    //     let incoming_stream = listener.incoming().next().unwrap().unwrap();
    //     connection.write("GET \n \r".as_bytes()).unwrap();

    //     handle_connection(incoming_stream);

    //     let buf_reader = BufReader::new(&mut connection);
    //     let http_request: Vec<String> = buf_reader
    //         .lines()
    //         .filter(|line| line.is_ok())
    //         .map(|result| result.unwrap())
    //         .collect();

    //     assert_eq!(format!("{:?}", http_request), "[\"HTTP/1.1 200 OK\", \"Content-Length: 26\", \"\", \"{\\\"npm-expansion\\\": \\\"hello\\\"}\"]");

    //     connection.flush().unwrap();
    //     connection.shutdown(std::net::Shutdown::Both).unwrap();
    // }

    // #[test]
    // fn handle_connection_non_real_route() {
    //     let (mut listener, mut connection) = setup_server_and_connection();
    //     let incoming_stream = listener.incoming().next().unwrap().unwrap();
    //     connection
    //         .write("GET /example-route HTTP/1.1 \n \r".as_bytes())
    //         .unwrap();

    //     handle_connection(incoming_stream);

    //     let buf_reader = BufReader::new(&mut connection);
    //     let http_request: Vec<String> = buf_reader
    //         .lines()
    //         .filter(|line| line.is_ok())
    //         .map(|result| result.unwrap())
    //         .collect();

    //     assert_eq!(
    //         format!("{:?}", http_request),
    //         "[\"HTTP/1.1 404 NOT FOUND\", \"\"]"
    //     );

    //     connection.flush().unwrap();
    //     connection.shutdown(std::net::Shutdown::Both).unwrap();
    // }
}
