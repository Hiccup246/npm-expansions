/// A HTTP response object containing the core parts of status line, headers and contents
pub struct HttpResponse {
    status_line: String,
    headers: String,
    contents: String,
}

impl HttpResponse {
    /// Creates a new response given a status line, headers and contents
    pub fn new(status_line: &str, headers: &str, contents: &str) -> HttpResponse {
        HttpResponse {
            status_line: status_line.to_string(),
            headers: headers.to_string(),
            contents: contents.to_string(),
        }
    }

    /// Returns the status line of the response object
    pub fn status_line(&self) -> &str {
        self.status_line.as_str()
    }

    /// Returns the headers of the response object
    pub fn headers(&self) -> &str {
        &self.headers
    }

    /// Returns the contents of the response object
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
    /// use npm_expansions::http_response::HttpResponse;
    ///
    /// let response = HttpResponse::new(
    ///     "200",
    ///     "Content-Type: application/json;q=0.5",
    ///     "Hello World!",
    /// ).into_bytes_vec();
    /// let example_response = "HTTP/1.1 200\r\nContent-Length: 12\r\nContent-Type: application/json;q=0.5\r\n\r\nHello World!".as_bytes().to_vec();
    ///
    /// assert_eq!(response, example_response);
    /// ```
    ///
    /// # Panics
    ///
    /// The function panics if string formatting via format! panics
    ///
    pub fn into_bytes_vec(self) -> Vec<u8> {
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
        fn correct_reponse_with_headers() {
            let response = HttpResponse::new(
                "200",
                "Content-Type: application/json;q=0.5",
                "Hello World!",
            );

            assert_eq!(response.into_bytes_vec(), "HTTP/1.1 200\r\nContent-Length: 12\r\nContent-Type: application/json;q=0.5\r\n\r\nHello World!".as_bytes().to_vec())
        }

        #[test]
        fn correct_reponse_without_headers() {
            let response = HttpResponse::new("200", "", "Hello World!");

            assert_eq!(
                response.into_bytes_vec(),
                "HTTP/1.1 200\r\nContent-Length: 12\r\n\r\nHello World!"
                    .as_bytes()
                    .to_vec()
            )
        }
    }
}
