use crate::toml;
use crate::run;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Read, Write};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;

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

fn handle_content(mut stream: TcpStream,
                  buf_reader: &mut BufReader<&TcpStream>,
                  content_length: usize) {
    let mut buffer = Vec::<u8>::new();
    buffer.resize(content_length, 0u8);
    buf_reader.read_exact(&mut buffer).unwrap();
    let body = String::from_utf8_lossy(&buffer).to_string();
    println!("montefile is:\n {}", body);
    let build_cfg = toml::read_build_cfg_from_string(&body).unwrap();

    let (send_chan, rec_chan): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(move || {
        run::run(build_cfg, true, Some(send_chan));
    });
    let headers = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/event-stream\r\n\
         Connection: keep-alive\r\n\
         Access-Control-Allow-Origin: *\r\n\
         \r\n"
    );
    stream.write_all(headers.as_bytes()).unwrap();
    stream.flush();
    while let Ok(line) = rec_chan.recv() {
        let data = format!(
            "{{\"status\": \"info\", \"message\": \"{}\"}}\n",
            line
        );
        stream.write_all(data.as_bytes()).unwrap();
        stream.flush();
    }
    let end = r#"{"status": "info": "message": "build succeeded"}"#;
    stream.write_all(end.as_bytes()).unwrap();
    stream.flush();
}

fn format_response(status: String, message: String) -> String {
    let response_body = format!(
        "{{\"status\": \"{}\", \"message\": \"{}\"}}",
        status, message
    );
    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json\r\n\
         Connection: keep-alive\r\n\
         Content-Size: {}
         \r\n\
         {}",
         response_body.len(),
         response_body
    )
}

fn handle_no_content(stream: &mut TcpStream) {
    let response = format_response(
        String::from("error"),
        String::from("empty config, skipping...")
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle(mut stream: TcpStream) {
    stream.set_read_timeout(Some(Duration::from_secs(30))).unwrap();
    let stream_clone = stream.try_clone().unwrap();
    let mut buf_reader = BufReader::new(&stream_clone);
    let headers = read_headers(&mut buf_reader);
    let content_length = get_content_length(&headers);
    if content_length > 0 {
        handle_content(stream, &mut buf_reader, content_length);
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
