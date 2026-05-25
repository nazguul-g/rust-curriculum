use colored::Colorize;
use std::io::{BufRead, BufReader, Write, stdin};
use std::net::TcpStream;
use std::{io, thread};

pub fn tcp_client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let reader = BufReader::new(stream.try_clone()?);

   thread::spawn(|| {
        for line in reader.lines() {
            match line {
                Ok(message) => {
                    println!("{message} {} ", "from server".bright_green())
                }
                Err(e) => {
                    println!("failed to fetch message from server, {e}")
                }
            }
        }
    });
    for line in stdin().lock().lines() {
        let msg = line?;
        if msg == "exit" {
            println!("👋 Disconnecting.");
            stream.write_all("goodbye".as_bytes())?;

            break;
        }
        stream.write_all(msg.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}
