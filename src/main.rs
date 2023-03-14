use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);

    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    // println!("{}", request_line);
    
    stream.write_all("HELLO".as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_connection_get_root() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        let mut incoming = TcpStream::connect("127.0.0.1:7878").unwrap();
        incoming.write("GET".as_bytes()).unwrap();
        // Write incoming request to listener
        let stream = listener.incoming().next().unwrap().unwrap();

        handle_connection(stream);
        
        let buf_reader = BufReader::new(&mut incoming);

        let request_line = buf_reader.lines().next().unwrap().unwrap();
        // Assert stream has correct response data

        assert_eq!(request_line, "");
    }
}