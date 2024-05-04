use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use std::time::Duration;
use protobuf::Message;
use futures_util::stream::{SplitSink, StreamExt};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Receiver;
use tokio::time::Instant;
use tokio_tungstenite::{accept_async, WebSocketStream};
use backend::gamelogic::GameController;
use backend::{MessageType, ServerOutput};
use backend::InputRequest;
use backend::PlayerId;

type TokioMessage = tokio_tungstenite::tungstenite::Message;
#[derive(Debug)]
struct NewPlayerConnection {
    id: i32,
    sink:  SplitSink<WebSocketStream<TcpStream>, TokioMessage>,
}
enum TxMessage  {
    SuccessfulConnection(NewPlayerConnection),
    PlayerInput(InputRequest),
    Disconnect(i32),
    Tick
}

#[tokio::main]
async fn main() {
    let (sender, receiver) = tokio::sync::mpsc::channel::<TxMessage>(100);
    let mut id_count:i32 = 0;

    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(addr).await.expect("Failed to start server");

    println!("Server running at {}!", addr);

    let game_ticker_send = sender.clone();

    tokio::spawn(main_game_loop(receiver));
    tokio::spawn(async move {
        let tick_rate = Duration::from_secs_f64(1.0 / 120.0); // 120 fps
        let mut last_tick = Instant::now();
        
        loop {
            let elapsed = Instant::now().duration_since(last_tick);
            if elapsed < tick_rate {
                tokio::time::sleep(tick_rate - elapsed).await;
            }
            last_tick = Instant::now();
            let _ = game_ticker_send.send(TxMessage::Tick).await;
        }
    });

    while let Ok((stream, _)) = listener.accept().await {
        id_count += 1;
        tokio::spawn(handle_connection(stream, sender.clone(), id_count));
    }
}

async fn main_game_loop(mut receiver: Receiver<TxMessage>) {
    let mut game_controller: GameController = GameController::new();
    let mut connection_pool: HashMap<i32, SplitSink<WebSocketStream<TcpStream>, TokioMessage>> = HashMap::<i32, SplitSink<WebSocketStream<TcpStream>, TokioMessage>>::new();

    while let Some(msg) = receiver.recv().await {
        match msg {
            TxMessage::SuccessfulConnection(mut new_connection) => {
                game_controller.add_player(new_connection.id);

                let mut new_player_message = PlayerId::new();
                new_player_message.set_field_type(MessageType::id_response);
                new_player_message.set_player_id(new_connection.id);

                let bytes = new_player_message.write_to_bytes().unwrap();
                let _ = new_connection.sink.send(TokioMessage::binary(bytes)).await;

                connection_pool.insert(new_connection.id, new_connection.sink);
                send_output_to_all_clients(connection_pool.values_mut(), game_controller.output()).await;
            },
            TxMessage::PlayerInput(input_request) => {
                game_controller.player_input(input_request);
            },
            TxMessage::Disconnect(player_id) => {
                game_controller.drop_player(player_id);
                connection_pool.remove_entry(&player_id);
                send_output_to_all_clients(connection_pool.values_mut(), game_controller.output()).await;
                println!("Connection dropped!")
            }, 
            TxMessage::Tick => {
                println!("Tick called!");
                if game_controller.should_tick() {
                    println!("and we actually ticked!");
                    game_controller.tick();
                    send_output_to_all_clients(connection_pool.values_mut(), game_controller.output()).await;
                }
            }   
        }
    }
}    

async fn send_output_to_all_clients(connections: ValuesMut<'_, i32, SplitSink<WebSocketStream<TcpStream>, TokioMessage>>, output: ServerOutput) {
    for write in connections {
        let _ = write.send(tokio_tungstenite::tungstenite::Message::binary(output.write_to_bytes().unwrap())).await;
    };
}

async fn handle_connection(stream: TcpStream, sender: Sender<TxMessage>, player_id: i32) {
    if let Err(e) = player_connection(stream, sender, player_id).await {
        println!("Something happened: {:?}", e)
    }
}

async fn player_connection(stream: TcpStream, sender: Sender<TxMessage>, player_id: i32) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let incoming_stream: WebSocketStream<TcpStream> = accept_async(stream).await.expect("Things went south during the handshake process");
    let (write, mut read) = incoming_stream.split();

    let _ = sender.send(TxMessage::SuccessfulConnection(NewPlayerConnection {id: player_id, sink: write})).await;
    println!("Connection established!");

    while let Some(Ok(msg)) = read.next().await {
        if msg.is_binary() {
            let input_request:InputRequest = InputRequest::parse_from_bytes(&msg.into_data()).unwrap();
            let _ = sender.send(TxMessage::PlayerInput(input_request)).await;
        }
    }
    let _ = sender.send(TxMessage::Disconnect(player_id)).await;
    Ok(())
}

