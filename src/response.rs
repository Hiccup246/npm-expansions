pub struct Response {
    status_line: String,
    headers: String,
    contents: String,
}

impl Response {
    pub fn new(status_line: &str, headers: &str, contents: String) -> Response {
        Response {
            status_line: status_line.to_string(),
            headers: headers.to_string(),
            contents,
        }
    }

    pub fn status_line(&self) -> &str {
        self.status_line.as_str()
    }

    pub fn headers(&self) -> &str {
        &self.headers
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let status = self.status_line();
        let contents = self.contents();
        let headers = self.headers();
        let length = contents.len();

        if headers.is_empty() {
            format!("HTTP/1.1 {status}\r\nContent-Length: {length}\r\n\r\n{contents}")
                .as_bytes()
                .to_vec()
        } else {
            format!("HTTP/1.1 {status}\r\nContent-Length: {length}\r\n{headers}\r\n\r\n{contents}")
                .as_bytes()
                .to_vec()
        }
    }

    // pub fn to_vec(&self) -> Vec<u8> {
    //     let status = self.status_line();
    //     let contents = self.contents();
    //     let headers = self.headers();
    //     let length = contents.len();
    //     let headers_vector:Vec<String> = headers.into_iter().map(|header| format!("{}: {}",header.0, header.1)).collect();
    //     let headers_string = headers_vector.join(",\r\n");

    //     format!("HTTP/1.1 {status}\r\nContent-Length: {length}\r\n{headers_string}\r\n\r\n{contents}").as_bytes().to_vec()
    // }
}
