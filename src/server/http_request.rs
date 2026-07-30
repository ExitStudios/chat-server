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
        let (header_part, body_part) = request.split_once("\r\n\r\n").unwrap_or((&request, ""));
        let mut lines = header_part.lines();
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

            let key = split.next().unwrap().trim();
            let value = split.next().unwrap_or("").trim();

            headers.insert(key.to_string(), value.to_string());
        }

        let body = body_part.as_bytes().to_vec();

        Self {
            method,
            path,
            version,
            headers,
            body,
        }
    }

    pub fn stringify_stream(reader: &mut impl BufRead) -> String {
        let mut request = String::new();

        // Read headers
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            request.push_str(&line);

            if line == "\r\n" {
                break;
            }
        }

        let content_length = request
            .lines()
            .find_map(|line| {
                line.strip_prefix("Content-Length:")
                    .map(|v| v.trim().parse::<usize>().unwrap())
            })
            .unwrap_or(0);

        // Read body
        if content_length > 0 {
            let mut body = vec![0; content_length];

            reader.read_exact(&mut body).unwrap();

            request.push_str(&String::from_utf8_lossy(&body));
        }

        request
    }
}
