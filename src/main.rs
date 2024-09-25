use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    //Read TcpStream
    let mut buffer: [u8; 1024] = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_n) => println!("Request: {}", String::from_utf8_lossy(&buffer[..])),
        Err(e) => panic!("Fail message: {}", e),
    };

    //Parse request header
    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    //Create response string
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    //Write response to client
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
