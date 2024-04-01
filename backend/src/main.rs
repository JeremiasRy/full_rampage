use std::collections::HashMap;
use std::process::Output;
use std::time::Duration;
use std::{borrow::Borrow, sync::Arc};
use mini_redis::Connection;
use tokio::sync::Mutex;
use futures_util::stream::{SplitSink, StreamExt};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};
use backend::gamelogic::{GameController, PlayerInputRequest};

type GameControllerArc = Arc<Mutex<GameController>>;
type TcpStreamWriteArc = Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>;
type ConnectionPool = Arc<Mutex<HashMap<i32, TcpStreamWriteArc>>>;

#[tokio::main]
async fn main() {
    let game_controller:GameControllerArc = Arc::new(Mutex::new(GameController::new()));
    let connection_pool: ConnectionPool = Arc::new(Mutex::new(HashMap::new()));

    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(addr).await.expect("Failed to start server");

    println!("Server running at {}!", addr);
    tokio::spawn(game_controller_updater(game_controller.clone(), connection_pool.clone()));
    while let Ok((stream, addr)) = listener.accept().await {
        println!("Incoming traffic {}", addr);

        let game_controller = game_controller.clone();
        let connection_pool = connection_pool.clone();
        tokio::spawn(handle_connection(stream, game_controller, connection_pool));
    }
}

async fn game_controller_updater(game_controller: GameControllerArc, connection_pool: ConnectionPool) {
    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        let mut controller = game_controller.lock().await;
        if controller.should_tick() {
            let connections = connection_pool.lock().await;
            controller.tick();
            println!("Tick!!");

            let output = controller.output();
            println!("{:?}", connections.iter().count());
            for write_arc in connections.values() {
                let mut write = write_arc.lock().await;
                let _ = write.send(Message::Text(String::from(serde_json::to_string(&output).unwrap()))).await;
            };
        }
    }
}

async fn handle_connection(stream: TcpStream, game_controller: GameControllerArc, connection_pool: ConnectionPool) {
    if let Err(e) = handle_connection_inner(stream, game_controller, connection_pool).await {
        println!("Something happened: {:?}", e)
    }
}

async fn handle_connection_inner(stream: TcpStream, game_controller: GameControllerArc, connection_pool: ConnectionPool) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let incoming_stream: WebSocketStream<TcpStream> = accept_async(stream).await.expect("Things went south during the handshake process");
    println!("Connection established!");

    let game_state = game_controller.clone();
    let (mut write, mut read) = incoming_stream.split();


    let mut connection_player_index = 0;

    { // add the new player to the game controller
        let mut controller = game_state.lock().await;
        connection_player_index = controller.add_player();
        let _ = write.send(Message::Text(connection_player_index.to_string())).await;
    }

    { // store the write to the connection pool so that we can send messages to it from the game state updater thread
        let mut connections = connection_pool.lock().await;
        connections.insert(connection_player_index, Arc::new(Mutex::new(write)));
    }

    while let Some(Ok(msg)) = read.next().await {
        if let Ok(json) = msg.into_text() {
            println!("{json}");
            let parsed_input = serde_json::from_str(&json);
            match parsed_input {
                Ok(res) => {
                    let mut controller = game_state.lock().await;
                    controller.player_input(res);
                },
                Err(e) => println!("Things went south: {:?}", e)
            }
        }
    }

    {
        let mut controller = game_state.lock().await;
        controller.drop_player(connection_player_index);
    }
    { // store the write to the connection pool so that we can send messages to it from the game state updater thread
        let mut connections = connection_pool.lock().await;
        connections.remove_entry(&connection_player_index);
    }
    Ok(())
}

