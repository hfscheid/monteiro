use crate::toml;
use crate::run;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Read, Write};

fn read_headers(buf_reader: &mut BufReader<&TcpStream>) -> Vec<String> {
    let mut headers = Vec::<String>::new();
    let mut line = String::new();
    loop {
        line.clear();
        if buf_reader.read_line(&mut line).unwrap() == 0 {
            break;
        }
        if line.trim().is_empty() {
            break;
        }
        headers.push(line.trim().to_string());
    }
    println!("Headers: {:#?}", headers);
    headers
}

fn get_content_length(headers: &Vec<String>) -> usize {
    let content_length = headers.iter()
        .find(|h| h.to_lowercase().starts_with("content-length:"))
        .and_then(|h| h.split_once(": "))
        .and_then(|(_, len)| len.parse::<usize>().ok())
        .unwrap_or(0);
    println!("Content length is {}", content_length);
    content_length
}

fn handle_content(
                  buf_reader: &mut BufReader<&TcpStream>,
                  content_length: usize) {
    let mut buffer = Vec::<u8>::new();
    buffer.resize(content_length, 0u8);
    buf_reader.read_exact(&mut buffer).unwrap();
    let body = String::from_utf8_lossy(&buffer).to_string();
    println!("montefile is:\n {}", body);
    let build_cfg = toml::read_build_cfg_from_string(&body).unwrap();
    run::run(build_cfg, true, None);
}

fn handle_no_content(stream: &mut TcpStream) {
    let response_body = String::from(
        "{\"status\": \"error\", \"message\": \"empty config, skipping...\"}"
    );
    let response = format!(
        "HTTP/1.1 400 Bad Request\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
         response_body.len(),
         response_body
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let headers = read_headers(&mut buf_reader);
    let content_length = get_content_length(&headers);
    if content_length > 0 {
        handle_content(&mut buf_reader, content_length);
    } else {
        handle_no_content(&mut stream);
    }
}

pub fn serve(port: i32) {
    println!("Running monteiro server on port {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}
