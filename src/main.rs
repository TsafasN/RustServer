use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = 
    match TcpListener::bind("127.0.0.1:7878") {
        Ok(t) => t,
        Err(e) => panic!("Fail message: {}", e),
    };

    for stream in listener.incoming() {
        let stream =
        match stream {
            Ok(t) => t,
            Err(e) => panic!("Fail message: {}", e),
        };

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
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    //Create response string
    let contents = 
    match fs::read_to_string(filename) {
        Ok(t) => t,
        Err(e) => panic!("Fail message: {}", e),
    };
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    //Write response to client
    match stream.write_all(response.as_bytes()) {
        Ok(_t) => (),
        Err(e) => panic!("Fail message: {}", e),
    }
    match stream.flush() {
        Ok(_t) => (),
        Err(e) => panic!("Fail message: {}", e),
    }
}
