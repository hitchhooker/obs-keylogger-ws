// server.rs
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:4444";
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut ws_sink, mut ws_stream) = ws_stream.split();

        while let Some(msg) = ws_stream.next().await {
            let msg = msg.unwrap();
            let text = msg.to_text().unwrap();
            println!("Received: {}", text);
            ws_sink.send(Message::text(text.to_owned())).await.unwrap();
        }
    }
}
