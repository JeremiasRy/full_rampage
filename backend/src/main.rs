use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use std::env;
use std::time::Duration;
use protobuf::Message;
use futures_util::stream::{SplitSink, StreamExt};
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Receiver;
use tokio::time::Instant;
use tokio_tungstenite::{accept_async, WebSocketStream};
use backend::gamelogic::{GameController, GameControllerTickOutput};
use backend::{ClientLobbyStatus, ClientRequestType, InputRequest, MessageType};
use backend::PlayerId;

type TokioMessage = tokio_tungstenite::tungstenite::Message;
#[derive(Debug)]
struct NewPlayerConnection {
    id: i32,
    sink:  SplitSink<WebSocketStream<TcpStream>, TokioMessage>,
}
enum TxMessage  {
    SuccessfulConnection(NewPlayerConnection),
    PlayerInGameInput(InputRequest),
    PlayerInLobbyInput(InputRequest),
    Disconnect(i32),
    Tick
}


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a framerate! Ex. ./backend 60, cargo run -- 60");
        return;
    }
    let frame_rate: f64 = args[1].parse().unwrap();

    println!("{}", frame_rate);
    let (sender, receiver) = tokio::sync::mpsc::channel::<TxMessage>(100);
    let mut id_count:i32 = 0;

    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(addr).await.expect("Failed to start server"); // TODO logging

    println!("Server running at {}!", addr); // TODO logging

    let game_ticker_send = sender.clone();

    tokio::spawn(main_game_loop(receiver));
    tokio::spawn(async move {
        let tick_rate = Duration::from_secs_f64(1.0 / frame_rate);
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
        // LOG incoming connection
        id_count += 1;
        tokio::spawn(handle_connection(stream, sender.clone(), id_count));
    }
}

async fn main_game_loop(mut receiver: Receiver<TxMessage>) {
    let mut game_controller: GameController = GameController::new();
    let mut connection_pool: HashMap<i32, SplitSink<WebSocketStream<TcpStream>, TokioMessage>> = HashMap::<i32, SplitSink<WebSocketStream<TcpStream>, TokioMessage>>::new();

    while let Some(msg) = receiver.recv().await {
        match msg {
            TxMessage::PlayerInLobbyInput(input) => {
                if input.get_status() == ClientLobbyStatus::ready {
                    game_controller.set_client_ready_for_war(input.player_id)
                }
                send_output_to_all_clients(connection_pool.values_mut(), game_controller.lobby_output()).await;
                if game_controller.clients_ready() {
                    game_controller.start_countdown();
                    send_output_to_all_clients(connection_pool.values_mut(), game_controller.in_game_output()).await;
                }
            },
            TxMessage::SuccessfulConnection(mut new_connection) => {
                game_controller.add_client(new_connection.id);

                let mut new_player_message = PlayerId::new();
                new_player_message.set_field_type(MessageType::id_response);
                new_player_message.set_player_id(new_connection.id);

                let bytes = new_player_message.write_to_bytes().unwrap();
                let _ = new_connection.sink.send(TokioMessage::binary(bytes)).await;
                if game_controller.is_counting_down() {
                    let _ = new_connection.sink.send(TokioMessage::binary(game_controller.in_game_output().write_to_bytes().unwrap())).await;
                }  

                connection_pool.insert(new_connection.id, new_connection.sink);
                send_output_to_all_clients(connection_pool.values_mut(), game_controller.lobby_output()).await;
                
            },
            TxMessage::PlayerInGameInput(input_request) => {
                game_controller.player_input(input_request);
            },
            TxMessage::Disconnect(player_id) => {
                game_controller.drop_client(player_id);
                connection_pool.remove_entry(&player_id);
                if !game_controller.is_playing() && !game_controller.is_counting_down() && game_controller.clients_ready()  {
                    game_controller.start_countdown();
                }
                send_output_to_all_clients(connection_pool.values_mut(), game_controller.in_game_output()).await;
                send_output_to_all_clients(connection_pool.values_mut(), game_controller.lobby_output()).await;
                println!("Connection dropped!") // TODO logging
            }, 
            TxMessage::Tick => {
                if game_controller.should_tick() {
                    match game_controller.tick() {
                        Some(GameControllerTickOutput::NotEnoughPlayers) => {
                            send_output_to_all_clients(connection_pool.values_mut(), game_controller.lobby_output()).await;
                        },
                        Some(GameControllerTickOutput::ScoreChanged) => {
                            send_output_to_all_clients(connection_pool.values_mut(), game_controller.lobby_output()).await;
                            send_output_to_all_clients(connection_pool.values_mut(), game_controller.in_game_output()).await;
                        },
                        Some(GameControllerTickOutput::WeHaveAWinner) => {
                            send_output_to_all_clients(connection_pool.values_mut(), game_controller.lobby_output()).await;
                            send_output_to_all_clients(connection_pool.values_mut(), game_controller.in_game_output()).await;
                        }
                        None => {
                            send_output_to_all_clients(connection_pool.values_mut(), game_controller.in_game_output()).await;
                        }
                    }
                } else if game_controller.is_counting_down() {
                    game_controller.countdown();
                    send_output_to_all_clients(connection_pool.values_mut(), game_controller.lobby_output()).await;
                }
            }   
        }
    }
}    

async fn send_output_to_all_clients<T : protobuf::Message>(connections: ValuesMut<'_, i32, SplitSink<WebSocketStream<TcpStream>, TokioMessage>>, output: T) {
    for write in connections {
        let _ = write.send(tokio_tungstenite::tungstenite::Message::binary(output.write_to_bytes().unwrap())).await;
    };
}

async fn handle_connection(stream: TcpStream, sender: Sender<TxMessage>, player_id: i32) {
    if let Err(e) = player_connection(stream, sender, player_id).await {
        println!("Something happened: {:?}", e) // TODO logging
    }
}

async fn player_connection(stream: TcpStream, sender: Sender<TxMessage>, player_id: i32) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let incoming_stream_result: Result<WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::Error> = accept_async(stream).await;
    match incoming_stream_result {
        Ok(incoming_stream) => {
            let (write, mut read) = incoming_stream.split();

            let _ = sender.send(TxMessage::SuccessfulConnection(NewPlayerConnection {id: player_id, sink: write})).await;
            println!("Connection established!"); // TODO logging

            while let Some(Ok(msg)) = read.next().await {
                if msg.is_binary() {

                    let input_request = InputRequest::parse_from_bytes(&msg.into_data()).unwrap();
                    match input_request.field_type {
                        ClientRequestType::in_game_input => {
                            let _ = sender.send(TxMessage::PlayerInGameInput(input_request)).await;
                        },
                        ClientRequestType::lobby_input => {
                            let _ = sender.send(TxMessage::PlayerInLobbyInput(input_request)).await;
                        },
                        ClientRequestType::empty_5 => {
                            // Shouldn't happen if it does lets do some logging
                        }
                    }
                }
            }
            let _ = sender.send(TxMessage::Disconnect(player_id)).await;
            Ok(())
        }, Err(error) => { // Let's do some logging here later
            println!("{:?}", error);
            Ok(())
        }
    }
}

