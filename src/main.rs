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
    stream.write_all("HELLO \r \n".as_bytes()).unwrap();
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
        let server_response = buf_reader.lines().next().unwrap().unwrap();

        assert_eq!(server_response, "HELLO \r ");
    }
}