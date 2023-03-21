use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub struct Request {
    pub status_line: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new<'a>(mut stream: &TcpStream) -> Request {
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

    pub fn status_line(&self) -> &str {
        self.status_line.as_str()
    }
}
