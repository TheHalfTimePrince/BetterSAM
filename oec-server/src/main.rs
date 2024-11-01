use tokio;
use warp::{ws::{Message, WebSocket}, Filter};
use futures::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum WSMessage {
    GenerateStream,
    StreamLink { url: String, session_id: String },
    Video(Vec<u8>),
    Chat(String),
    Location { latitude: f64, longitude: f64 },
}

type Users = Arc<Mutex<HashMap<String, futures::channel::mpsc::UnboundedSender<Message>>>>;

#[tokio::main]
async fn main() {
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| {
            ws.on_upgrade(move |socket| handle_websocket(socket, users))
        });

    println!("WebSocket server starting on ws://0.0.0.0:8080/ws");
    warp::serve(ws_route).run(([0, 0, 0, 0], 8080)).await;
}

async fn handle_websocket(ws: WebSocket, users: Users) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (user_tx, mut user_rx) = futures::channel::mpsc::unbounded();
    let user_id = uuid::Uuid::new_v4().to_string();

    // Store user connection
    users.lock().unwrap().insert(user_id.clone(), user_tx);

    // Forward messages from user_rx to the WebSocket sender
    let users_clone = users.clone();
    let user_id_clone = user_id.clone();

    tokio::spawn(async move {
        while let Some(message) = ws_receiver.next().await {
            match message {
                Ok(msg) => {
                    if msg.is_binary() {
                        println!("Received binary message of size: {} bytes", msg.as_bytes().len());
                        // Forward binary message to all other users
                        let users = users_clone.lock().unwrap();
                        for (id, tx) in users.iter() {
                            if *id != user_id_clone {
                                if let Err(e) = tx.unbounded_send(msg.clone()) {
                                    eprintln!("Error forwarding binary message: {:?}", e);
                                }
                            }
                        }
                    } else if let Ok(text) = msg.to_str() {
                        println!("Received text message: {}", text);
                        if let Ok(ws_message) = serde_json::from_str::<WSMessage>(text) {
                            println!("Parsed message: {:?}", ws_message);
                            match ws_message {
                                WSMessage::GenerateStream => {
                                    println!("Generating stream link");
                                    let session_id = uuid::Uuid::new_v4().to_string();
                                    let sender_url = format!("/sender?session={}", session_id);
                                    
                                    let response = WSMessage::StreamLink { 
                                        url: sender_url.clone(),
                                        session_id: session_id 
                                    };
                                    println!("Sending response: {:?}", response);
                                    let msg = Message::text(serde_json::to_string(&response).unwrap());
                                    if let Some(tx) = users_clone.lock().unwrap().get(&user_id_clone) {
                                        if let Err(e) = tx.unbounded_send(msg) {
                                            eprintln!("Error sending stream link: {:?}", e);
                                        }
                                    }
                                },
                                WSMessage::StreamLink { .. } => (),
                                WSMessage::Video(video) => {
                                    let users = users_clone.lock().unwrap();
                                    for (id, tx) in users.iter() {
                                        if *id != user_id_clone {
                                            let msg = Message::text(serde_json::to_string(&WSMessage::Video(video.clone())).unwrap());
                                            if let Err(e) = tx.unbounded_send(msg) {
                                                eprintln!("Error sending video message: {:?}", e);
                                            }
                                        }
                                    }
                                },
                                WSMessage::Chat(chat) => {
                                    let users = users_clone.lock().unwrap();
                                    for (id, tx) in users.iter() {
                                        if *id != user_id_clone {
                                            let msg = Message::text(serde_json::to_string(&WSMessage::Chat(chat.clone())).unwrap());
                                            if let Err(e) = tx.unbounded_send(msg) {
                                                eprintln!("Error sending chat message: {:?}", e);
                                            }
                                        }
                                    }
                                },
                                WSMessage::Location { latitude, longitude } => {
                                    let users = users_clone.lock().unwrap();
                                    for (id, tx) in users.iter() {
                                        if *id != user_id_clone {
                                            let msg = Message::text(serde_json::to_string(&WSMessage::Location { latitude, longitude }).unwrap());
                                            if let Err(e) = tx.unbounded_send(msg) {
                                                eprintln!("Error sending location message: {:?}", e);
                                            }
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket error: {:?}", e);
                    break;
                }
            }
        }
    });

    // Forward messages from user_rx to ws_sender
    while let Some(message) = user_rx.next().await {
        if let Err(e) = ws_sender.send(message).await {
            eprintln!("Error sending message: {:?}", e);
            break;
        }
    }

    // Clean up on disconnect
    users.lock().unwrap().remove(&user_id);
}
