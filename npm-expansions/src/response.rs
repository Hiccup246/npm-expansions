pub struct Response {
    status_line: String,
    headers: String,
    contents: String,
}

impl Response {
    pub fn new(status_line: &str, headers: &str, contents: &str) -> Response {
        Response {
            status_line: status_line.to_string(),
            headers: headers.to_string(),
            contents: contents.to_string(),
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

    /// Converts a request object into a http response vector of bytes
    ///
    /// # Arguments
    ///
    /// * `&self` - Current instantiation of the Response struct
    ///
    /// # Examples
    ///
    /// ```
    /// let response = Response::new(
    ///     "200",
    ///     "Content-Type: application/json;q=0.5",
    ///     "Hello World!".to_string(),
    /// ).into_http_response();
    /// let example_response = "HTTP/1.1 200\r\nContent-Length: 12\r\nContent-Type: application/json;q=0.5\r\n\r\nHello World!".as_bytes().to_vec();
    ///
    /// assert_eq!(response, example_response)
    /// ```
    ///
    /// # Panics
    ///
    /// The function panics if string formatting via format! panics
    ///
    pub fn into_http_response(self) -> Vec<u8> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod into_http_response {
        use super::*;

        #[test]
        fn correctly_reponse_with_headers() {
            let response = Response::new(
                "200",
                "Content-Type: application/json;q=0.5",
                "Hello World!",
            );

            assert_eq!(response.into_http_response(), "HTTP/1.1 200\r\nContent-Length: 12\r\nContent-Type: application/json;q=0.5\r\n\r\nHello World!".as_bytes().to_vec())
        }

        #[test]
        fn correctly_reponse_without_headers() {
            let response = Response::new("200", "", "Hello World!");

            assert_eq!(
                response.into_http_response(),
                "HTTP/1.1 200\r\nContent-Length: 12\r\n\r\nHello World!"
                    .as_bytes()
                    .to_vec()
            )
        }
    }
}
