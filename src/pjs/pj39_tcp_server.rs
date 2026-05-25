use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process::exit;
use std::{io, thread};

pub fn tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for listener in listener.incoming() {
        match listener {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("connection failed , {e}");
            }
        }
    }
    Ok(())
}
fn handle_client(mut stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());

    for line in reader.lines() {
        match line {
            Ok(message) => match message.to_lowercase().as_str().trim() {
                "quit" => {
                    stream.write_all("Goodbye!".as_bytes()).unwrap();
                    stream.shutdown(Shutdown::Both).unwrap();
                    break
                }
                _ => {
                    println!("{peer} : {message}");
                    let wrt = format!("Echo: {message}\n");
                    stream.write_all(wrt.as_bytes()).unwrap();
                }
            },
            Err(e) => {
                println!("failed to read the message from stream, {e}")
            }
        }
    }
    println!("connection to {peer} ended")
}
