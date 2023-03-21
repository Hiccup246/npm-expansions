use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub struct Request {
    status_line: String,
    headers: HashMap<String, String>,
}

impl Request {
    pub fn build<'a>(mut stream: &TcpStream) -> Request {
        let buf_reader = BufReader::new(&mut stream);
        let mut buffer = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty());

        let status_line = buffer.next().unwrap();

        let mut headers: HashMap<String, String> = HashMap::new();

        for header in buffer {
            let (key, value) = header.split_at(header.find(":").unwrap());
            let (_colon, header_value) = value.split_at(1);
            headers.insert(key.to_string(), header_value.trim().to_string());
        }

        Request {
            status_line,
            headers,
        }
    }

    pub fn new<'a>(status_line: &str, headers: HashMap<String, String>) -> Request {
        Request {
            status_line: status_line.to_string(),
            headers: headers.clone(),
        }
    }

    pub fn status_line(&self) -> &str {
        self.status_line.as_str()
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};

    fn setup_server_and_connection() -> (TcpListener, TcpStream) {
        (
            TcpListener::bind("127.0.0.1:7878").unwrap(),
            TcpStream::connect("127.0.0.1:7878").unwrap(),
        )
    }

    #[test]
    fn parses_status_line() {
        let (listener, mut connection) = setup_server_and_connection();
        let incoming_stream = listener.incoming().next().unwrap().unwrap();

        connection.write("GET / HTTP/1.1\n\r".as_bytes()).unwrap();

        let request = Request::build(&incoming_stream);

        connection.flush().unwrap();
        connection.shutdown(std::net::Shutdown::Both).unwrap();

        assert_eq!(request.status_line(), "GET / HTTP/1.1")
    }

    #[test]
    fn parses_headers() {
        let (listener, mut connection) = setup_server_and_connection();
        let incoming_stream = listener.incoming().next().unwrap().unwrap();

        connection.write("GET / HTTP/1.1\n\rAccept: application/text,text/plain;q=0.1\r\nContent-Length: 0\r\n\r\n".as_bytes()).unwrap();

        let request = Request::build(&incoming_stream);

        connection.flush().unwrap();
        connection.shutdown(std::net::Shutdown::Both).unwrap();

        assert!(request.headers().eq(&HashMap::from([
            (
                "Accept".to_string(),
                "application/text,text/plain;q=0.1".to_string()
            ),
            ("Content-Length".to_string(), "0".to_string()),
        ])))
    }
}
