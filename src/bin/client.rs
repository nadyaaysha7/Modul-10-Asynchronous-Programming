use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Branch 1: We receive an incoming message from the server
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From server: {}", text);
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => {
                        println!("Server disconnected.");
                        return Ok(());
                    }
                }
            }
            // Branch 2: The user types a message in the terminal
            res = stdin.next_line() => {
                match res {
                    Ok(Some(line)) => {
                        // Send the typed message to the server via websocket
                        ws_stream.send(Message::text(line)).await?;
                    }
                    Ok(None) => return Ok(()), // EOF (Ctrl+D)
                    Err(err) => {
                        println!("Error reading from stdin: {err}");
                        return Ok(());
                    }
                }
            }
        }
    }
}