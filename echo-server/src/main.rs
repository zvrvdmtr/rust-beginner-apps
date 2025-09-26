use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};

enum HTTPVerb {
    Get,
    Post,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .expect("Can't set read timeout");
                stream
                    .set_write_timeout(Some(Duration::from_secs(5)))
                    .expect("Can't set write timeout");
                handle_client(stream);
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = BufReader::new(&stream);
    let headers: Vec<_> = buffer
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = match get_http_verb(&headers) {
        Some(HTTPVerb::Get) => handle_get(),
        Some(HTTPVerb::Post) => handle_post(&headers, &mut buffer),
        None => handler_not_allowed(),
    };
    stream.write(response.as_bytes()).unwrap();
}

fn handler_not_allowed() -> String {
    "HTTP/1.1 405 Method not allowed\r\nConnection: close\r\n\r\n".to_string()
}

fn handle_get() -> String {
    return "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string();
}

fn handle_post(headers: &[String], buffer: &mut BufReader<&TcpStream>) -> String {
    let content_type = match get_content_type(&headers) {
        Some(value) => value,
        None => "text/plain".to_string(),
    };

    let content_length = match get_content_length(&headers) {
        Some(value) => value,
        None => {
            return format!(
                "HTTP/1.1 411 Length Required\r\nContent-Length: 0\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
                content_type
            );
        }
    };

    let mut body = vec![0; content_length];
    match buffer.read_exact(&mut body) {
        Ok(_) => {
            return format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                content_type,
                String::from_utf8_lossy(&body)
            );
        }
        Err(err) => {
            return format!(
                "HTTP/1.1 408 Request Timeout\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n{}",
                err.to_string().len(),
                content_type,
                err.to_string()
            );
        }
    }
}

fn get_http_verb(headers: &[String]) -> Option<HTTPVerb> {
    headers.iter().find_map(|header| {
        match header.split_whitespace().next()?.to_uppercase().as_str() {
            "GET" => Some(HTTPVerb::Get),
            "POST" => Some(HTTPVerb::Post),
            _ => None,
        }
    })
}

fn get_content_length(headers: &[String]) -> Option<usize> {
    let content_length = match try_get_header(headers, "content-length:") {
        Some(value) => value.parse::<usize>().ok(),
        None => None,
    };
    content_length
}

fn get_content_type(headers: &[String]) -> Option<String> {
    return match try_get_header(headers, "content-type:") {
        Some(value) => Some(value),
        None => None,
    };
}

fn try_get_header(headers: &[String], title: &str) -> Option<String> {
    headers.iter().find_map(|header| {
        header
            .to_lowercase()
            .strip_prefix(title)
            .map(|value| value.trim().to_string())
    })
}
