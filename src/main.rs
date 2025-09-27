use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // запускаємо сервер на порту 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🚀 Сервер запущено на http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // простий роутинг: "/" -> index.html, "/hello" -> hello.html
    let get_root = b"GET / HTTP/1.1\r\n";
    let get_hello = b"GET /hello HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(get_hello) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| "<h1>Error loading file</h1>".to_string());
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
