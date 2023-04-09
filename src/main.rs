use device_query::{DeviceQuery, DeviceState};
use futures_util::SinkExt;
use serde::Serialize;
use std::env;
use tokio::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Serialize)]
struct ObsCommand {
    request_type: String,
    item: String,
    #[serde(rename = "sourceSettings[text]")]
    text: String,
    message_id: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <obs_url>", args[0]);
        return;
    }

    let obs_url = &args[1];
    let device_state = DeviceState::new();

    let (mut obs_ws, _) = connect_async(obs_url)
        .await
        .expect("Failed to connect to OBS WebSocket");

    loop {
        let keys = device_state.get_keys();
        for key in keys {
            let key_text = format!("{:?}", key);

            let command = ObsCommand {
                request_type: "SetTextGDIPlusProperties".to_string(),
                item: "KeyloggerText".to_string(),
                text: key_text,
                message_id: "1".to_string(),
            };

            let command_json = serde_json::to_string(&command).unwrap();
            obs_ws.send(Message::text(command_json)).await.unwrap();
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
