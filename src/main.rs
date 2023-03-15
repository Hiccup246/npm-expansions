use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let expansion = "";
    let contents = "{\"npm-expansion\": \"hello\"}";
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}\r\n");
    println!("{}", response);
    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_server_and_connection() -> (TcpListener, TcpStream) {
        (TcpListener::bind("127.0.0.1:7878").unwrap(), TcpStream::connect("127.0.0.1:7878").unwrap())
    }

    #[test]
    fn handle_connection_get_root() {
        let (mut listener, mut connection) = setup_server_and_connection();
        let incoming_stream = listener.incoming().next().unwrap().unwrap();
        connection.write("GET \n \r".as_bytes()).unwrap();    

        handle_connection(incoming_stream);

        let buf_reader = BufReader::new(&mut connection);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap_or("default".to_string()))
            .collect();
        
        assert_eq!(format!("{:?}", http_request), "[\"HTTP/1.1 200 OK\", \"Content-Length: 26\", \"\", \"{\\\"npm-expansion\\\": \\\"hello\\\"}\", \"default\"]");
    }
}