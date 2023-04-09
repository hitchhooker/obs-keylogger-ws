use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::env;
use device_query::{DeviceQuery, DeviceState, Keycode};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[derive(Debug, Clone, Copy)]
struct KeyWrapper(Keycode);

impl PartialEq for KeyWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 as u32 == other.0 as u32
    }
}

impl Eq for KeyWrapper {}

impl Hash for KeyWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0 as u32).hash(state);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <server_url>", args[0]);
        return;
    }

    let server_url = &args[1];
    let device_state = DeviceState::new();
    let previous_keys = Arc::new(Mutex::new(HashSet::new()));

    let listener = TcpListener::bind(server_url).await.unwrap();
    println!("Listening on: {}", server_url);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut ws_sink, mut ws_stream) = ws_stream.split();

        let prev_keys_clone = previous_keys.clone();
        tokio::spawn(async move {
            while let Some(_) = ws_stream.next().await {
                let keys = device_state.get_keys();
                let current_keys: HashSet<_> = keys.into_iter().map(KeyWrapper).collect();

                let mut prev_keys_guard = prev_keys_clone.lock().unwrap();
                let new_keys = current_keys.difference(&*prev_keys_guard).cloned().collect::<HashSet<_>>();

                for key in new_keys {
                    println!("New key pressed: {:?}", key.0);
                    ws_sink.send(Message::text(format!("{:?}", key.0))).await.unwrap();
                }

                *prev_keys_guard = current_keys;
            }
        });
    }
}
