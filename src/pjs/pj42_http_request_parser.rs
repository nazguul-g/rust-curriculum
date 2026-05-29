use std::io::{ Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{io, thread};

/// request
///  ```
///  GET /index.html HTTP/1.1\r\n
///  Host: example.com\r\n
///  User-Agent: Mozilla/5.0\r\n
///  \r\n
///  [body here, for POST requests]
/// ```
/// request
///  ```
///  HTTP/1.1 200 OK\r\n
///  Content-Type: text/html\r\n
///  \r\n
///  <html>hello</html>
/// ```

// a server listens to client request and respond with http style
pub fn http_parser() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|e| eprintln!("error handling the client, {e}"));
        });
    }

    Ok(())
}
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer);
    let lines: Vec<_> = request.lines().collect();
    if let Some(r_line) = lines.first() {
        println!("request {}", r_line);
    }

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h1>Hello from Rust HTTP Server!</h1></body></html>";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
