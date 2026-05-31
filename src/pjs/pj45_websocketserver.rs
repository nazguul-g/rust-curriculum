use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
pub async fn websockets() {
    let listener = TcpListener::bind("127.0.0.1:9001")
        .await
        .expect("failed to start server");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("new connection from {addr}");

        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    let (mut write, mut read) = ws_stream.split();
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                println!("received message : {text}");
                                if let Err(e) = write.send(Message::Text(text)).await {
                                    eprintln!("send error , {e}");
                                    break;
                                }
                            }
                            Ok(Message::Binary(bytes)) => {
                                println!("received binary payload");
                                if let Err(e) = write.send(Message::Binary(bytes)).await {
                                    eprintln!("send error , {e}");
                                    break;
                                }
                            }
                            Ok(Message::Close(_)) => {
                                println!("client disconnected");
                                break;
                            }
                            Ok(Message::Ping(ping)) => {
                                println!("server sent ping message");
                                if let Err(e) = write.send(Message::Pong(ping)).await {
                                    eprintln!("send error , {e}");
                                    break;
                                }
                            }
                            Ok(_) => {}
                            Err(e) => {}
                        }
                    }
                }
                Err(e) => println!("websockets handshake failed. {e}"),
            }
        });
    }
}
