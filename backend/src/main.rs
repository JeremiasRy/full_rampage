use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tokio_tungstenite::WebSocketStream;
use futures_util::{SinkExt, StreamExt};
use backend::gamelogic::{GameController, PlayerInputRequest};

#[tokio::main]
async fn main() {
    let game_state = Arc::new(Mutex::new(GameController::new()));
    let connections = Arc::new(Mutex::new(HashMap::<usize, WebSocketStream<TcpStream>>::new()));

    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(addr).await.expect("Failed to start server");

    println!("Server running at {}!", addr);

    tokio::spawn(connection_handler(listener, game_state.clone(), connections.clone()));
    tokio::spawn(game_state_updater(game_state.clone(), connections.clone()));
}

async fn game_state_updater(game_state: Arc<Mutex<GameController>>, connections: Arc<Mutex<HashMap<usize, WebSocketStream<TcpStream>>>>) {
    loop {
        let mut controller = game_state.lock().await;

        if controller.should_tick() {
            controller.tick();

            println!("send new game state to all clients, also limit fps to 60?")
        }
    }
}

async fn connection_handler(listener: TcpListener, game_state: Arc<Mutex<GameController>>, connections: Arc<Mutex<HashMap<usize, WebSocketStream<TcpStream>>>>) {
    loop {
        let (stream, _) = listener.accept().await.expect("Error happened when receiving a request");

        let game_state = game_state.clone();
        tokio::spawn(handle_connection(stream, game_state));
    }
}

async fn handle_connection(stream: TcpStream, game_state: Arc<Mutex<GameController>>) {
    if let Err(e) = handle_connection_inner(stream, game_state).await {
        println!("Something happened: {:?}", e)
    }
}

async fn handle_connection_inner(stream: TcpStream, game_state: Arc<Mutex<GameController>>) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let incoming_stream = accept_async(stream).await.expect("Things went south during the handshake process");
    println!("Connection established!");

    let game_state = game_state.clone();

    let (mut write, mut read) = incoming_stream.split();

    while let Some(Ok(msg)) = read.next().await {
        let mut controller = game_state.lock().await;
        if msg.is_text() {
            let json_str = msg.into_text().unwrap();
            println!("received player input: {}", json_str);

            if let Ok(player_input) = serde_json::from_str::<PlayerInputRequest>(&json_str) {
                controller.player_input(player_input);
            }
        } else {
            println!("New player!!");
            let id_response = controller.add_player();
            write.send(Message::text(id_response.to_string())).await?;
        }
    }

    Ok(())
}

