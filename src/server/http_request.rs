use std::{collections::HashMap, io::BufRead};

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn from_request_string(request: String) -> Self {
        let mut lines = request.lines();

        // First line: GET / HTTP/1.1
        let request_line = lines.next().unwrap();

        let mut parts = request_line.split_whitespace();

        let method = match parts.next().unwrap() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            other => panic!("Unsupported method: {}", other),
        };

        let path = parts.next().unwrap().to_string();

        let version = parts.next().unwrap().replace("HTTP/", "");

        let mut headers = HashMap::new();

        for line in lines {
            let mut split = line.splitn(2, ':');

            let key = match split.next() {
                Some(value) => value,
                None => continue,
            };

            let value = split.next().unwrap_or("").trim();

            headers.insert(key.to_string(), value.to_string());
        }

        Self {
            method,
            path,
            version,
            headers,
            body: Vec::new(),
        }
    }

    pub fn stringify_stream(reader: &mut impl BufRead) -> String {
        let mut request = String::new();

        for line in reader.lines() {
            let line = line.unwrap();

            // End of HTTP headers
            if line.is_empty() {
                break;
            }

            request.push_str(&line);
            request.push('\n');
        }

        request
    }
}
