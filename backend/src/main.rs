use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;
use std::sync::Arc;
use protobuf::{Message, RepeatedField};
use tokio::sync::Mutex;
use futures_util::stream::{SplitSink, StreamExt};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::Instant;
use tokio_tungstenite::{accept_async, WebSocketStream};
use backend::gamelogic::GameController;
use backend::ServerOutput;
use backend::ControllerResponse;
use backend::InputRequest;
use backend::PlayerId;


type GameControllerArc = Arc<Mutex<GameController>>;
type TcpStreamWriteArc = Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::Message>>>;
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
    let tick_rate = Duration::from_secs_f64(1.0 / 60.0); // 60 fps
    let mut last_tick = Instant::now();
    loop {

        let elapsed = Instant::now().duration_since(last_tick);
        if elapsed < tick_rate {
            tokio::time::sleep(tick_rate - elapsed).await;
        }
        last_tick = Instant::now();

        let mut controller = game_controller.lock().await;
        if controller.should_tick() {
            let connections = connection_pool.lock().await;
            controller.tick();

            let output: ServerOutput = controller.output();

            for write_arc in connections.values() {
                let mut write = write_arc.lock().await;
                let _ = write.send(tokio_tungstenite::tungstenite::Message::binary(output.write_to_bytes().unwrap())).await;
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
        
        let mut message = PlayerId::new();
        message.set_player_id(connection_player_index);
        
        let bytes = message.write_to_bytes().unwrap();
        let _ = write.send(tokio_tungstenite::tungstenite::Message::Binary(bytes)).await;
    }

    { // store the write to the connection pool so that we can send messages to it from the game state updater thread
        let mut connections = connection_pool.lock().await;
        connections.insert(connection_player_index, Arc::new(Mutex::new(write)));
    }

    while let Some(Ok(msg)) = read.next().await {
        if msg.is_binary() {
            let input_request:InputRequest = InputRequest::parse_from_bytes(&msg.into_data()).unwrap();
            println!("input player id: {}, input i32: {}", input_request.player_id, input_request.input);

            let mut controller = game_state.lock().await;
            controller.player_input(input_request.player_id, input_request.input.try_into().unwrap());
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

