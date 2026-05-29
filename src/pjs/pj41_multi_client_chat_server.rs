use std::collections::HashMap;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{io, thread};

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;
pub fn chat_server() -> io::Result<()> {
    // bind new server
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    // construct new clients
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_add = stream.peer_addr()?.to_string();
                let clients = Arc::clone(&clients);
                clients
                    .lock()
                    .unwrap()
                    .insert(client_add.clone(), stream.try_clone()?);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, &client_add, clients) {
                        eprintln!("error handling client,{e}")
                    }
                });
            }
            Err(e) => {
                println!("error receiving streams , {e}");
                break;
            }
        }
    }
    Ok(())
}

fn handle_client(stream: TcpStream, addr: &str, clients: Clients) -> io::Result<()> {
    let stream = stream.try_clone()?;
    let reader = BufReader::new(stream);
    // accepting messages
    for line in reader.lines() {
        let mut disconnected_clients = vec![];
        // TODO: add sender addresses and improve error message
        let message = match line {
            Ok(message) => {
                println!("received message : {message}");
                message
            }
            Err(e) => {
                println!("Error reading line : {e}");
                break;
            }
        };
        // sending messages

        let mut clients_lock = clients.lock().unwrap();
        for (peer, client_stream) in clients_lock.iter_mut() {
            // prevent reading same client message
            if addr != peer {
                if let Err(e) = writeln!(client_stream, "{}", message) {
                    disconnected_clients.push(peer.clone());
                }
            }
        }
        for peer in disconnected_clients {
            clients_lock.remove(&peer);
        }
    }
    clients.lock().unwrap().remove(addr);

    Ok(())
}
