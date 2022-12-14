use std::net::{
    TcpListener,
    TcpStream
};
use std::io::{
    Read,
    Write
};
use std::fs;

fn main() {
    let listener: TcpListener =
        TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("Webserver started");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connected");
        handle_connection(stream);
    };
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();
        let res = format!(
            "HTTP/1.1 200 OK\r\ncontent length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents =
            fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\ncontent length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
