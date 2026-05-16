use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    // Send a welcome message to the newly connected client
    ws_stream.send(Message::text("Welcome to chat! Type a message")).await?;

    // Subscribe to the broadcast channel
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Branch 1: We receive a message from THIS client's websocket
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From client {addr:?} {text:?}");
                            // Broadcast the message to all other connected clients
                            let _ = bcast_tx.send(text.into());
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()), // Client disconnected
                }
            }
            // Branch 2: We receive a message from the broadcast channel (sent by another client)
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(text) => ws_stream.send(Message::text(text)).await?,
                    Err(err) => return Err(err.into()),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await.unwrap();

            if let Err(e) = handle_connection(addr, ws_stream, bcast_tx).await {
                println!("Error handling connection from {addr:?}: {e}");
            }
        });
    }
}