use std::sync::Arc;
use tokio::sync::Mutex;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};
use backend::gamelogic::GameController;

type GameControllerArc = Arc<Mutex<GameController>>;

#[tokio::main]
async fn main() {
    let game_controller:GameControllerArc = Arc::new(Mutex::new(GameController::new()));

    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(addr).await.expect("Failed to start server");

    println!("Server running at {}!", addr);

    let _ = tokio::spawn(connection_handler(listener, game_controller.clone())).await;
}

async fn connection_handler(listener: TcpListener, game_controller: GameControllerArc) {
    loop {
        let (stream, _) = listener.accept().await.expect("Error happened when receiving a request");

        let game_controller = game_controller.clone();
        tokio::spawn(handle_connection(stream, game_controller));
    }
}

async fn handle_connection(stream: TcpStream, game_controller: GameControllerArc) {
    if let Err(e) = handle_connection_inner(stream, game_controller).await {
        println!("Something happened: {:?}", e)
    }
}

async fn handle_connection_inner(stream: TcpStream, game_controller: GameControllerArc) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let incoming_stream: WebSocketStream<TcpStream> = accept_async(stream).await.expect("Things went south during the handshake process");
    println!("Connection established!");

    let game_state = game_controller.clone();

    let (mut write, mut read) = incoming_stream.split();

    let mut controller = game_state.lock().await;
    let result = write.send(Message::Text(controller.add_player().to_string())).await;

    println!("result {:?}", result);

    while let Some(Ok(msg)) = read.next().await {
        if msg.is_text() {
            println!("{msg}");
        }
    }
    println!("Connection dropped!");
    Ok(())
}

