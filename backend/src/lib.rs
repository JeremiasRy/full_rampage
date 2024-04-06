include!(concat!(env!("OUT_DIR"), "/messages.rs"));

pub mod gamelogic {
    use bitflags::bitflags;
    use protobuf::RepeatedField;
    use rand::{thread_rng, Rng};
    use serde::Serialize;
    use tokio_tungstenite::tungstenite::http::response;
    const PLAYER_SIZE: i32 = 25;
    const CANNON_LENGTH: i32 = 25;
    type ServerOutput = crate::ServerOutput;
    type ControllerResponse = crate::ControllerResponse;
    type InputRequest = crate::InputRequest;
    type PlayerId = crate::PlayerId;
    type Point = crate::Point;

    bitflags! {
        #[derive(Debug)]
        struct PlayerInputFlags: u8 {
            const noinput = 0;
            const up = 0b000001;
            const right = 0b000010;
            const down = 0b000100;
            const left = 0b001000;
            const cannon_positive = 0b010000;
            const cannon_negative = 0b100000;
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Serialize)]
    struct ControllerPoint {
        x: i32,
        y: i32
    }

    impl ControllerPoint {
        pub fn translate(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y += dy;
        }
        pub fn random_point(height_bounds: i32, width_bounds: i32) -> ControllerPoint {
            ControllerPoint {
                x: thread_rng().gen_range(0..=width_bounds),
                y: thread_rng().gen_range(0..=height_bounds)
            }
        }
        pub fn to_buffer_point(&self) -> Point {
            let mut point = Point::new();
            point.set_x(self.x);
            point.set_y(self.y);

            point
        }
    }

    #[derive(Debug)]
    struct Player {
        id: i32,
        position: ControllerPoint,
        cannon_angle: i32,
        input: PlayerInputFlags,
    }

    impl Player {
        pub fn has_input(&self) -> bool {
            self.input.bits() > 0
        }

        pub fn tick(&mut self) {
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;
            let mut da: i32 = 0;

            if self.input.contains(PlayerInputFlags::up) {
                dy -= 1;
            }
            if self.input.contains(PlayerInputFlags::down) {
                dy += 1;
            }
            if self.input.contains(PlayerInputFlags::right) {
                dx += 1;
            }
            if self.input.contains(PlayerInputFlags::left) {
                dx -= 1;
            }
            if self.input.contains(PlayerInputFlags::cannon_positive) {
                da += 1;
            }
            if self.input.contains(PlayerInputFlags::cannon_negative) {
                da -= 1;
            }

            self.position.x += dx;
            self.position.y += dy;
            let mut new_angle = (self.cannon_angle + da) % 359;

            if new_angle < 0 {
                new_angle += 360;
            }
            self.cannon_angle = new_angle;
        }

        fn calculate_cannon_position(&self) -> ControllerPoint {
            let cannon_radians = (self.cannon_angle as f32).to_radians();
            let (center_x, center_y) = (self.position.x + PLAYER_SIZE / 2, self.position.y + PLAYER_SIZE / 2);
            let dx = CANNON_LENGTH as f32 * cannon_radians.cos();
            let dy = CANNON_LENGTH as f32 * cannon_radians.sin();
            ControllerPoint {
                x: dx.round() as i32 + center_x,
                y: dy.round() as i32 + center_y
            }
        }
    }

    pub struct GameController {
        height: i32,
        width: i32,
        players: Vec<Player>,
        id_count: i32
    }

    impl GameController {
        pub fn new() -> GameController {
            GameController {
                id_count: 0,
                height: 800,
                width: 1200,
                players: Vec::<Player>::new()
            }
        }
        pub fn tick(&mut self) {
            for player in self.players.iter_mut() {
                if player.has_input() {
                    player.tick()
                }
            }
        }
        pub fn should_tick(&self) -> bool { // for now lets just check if players have some input in the future need to check for particles etc..
            self.players.iter().any(|player| player.has_input())
        }
        pub fn player_input(&mut self, player_id: i32, input: u8) {
            let player: &mut Player = self.get_player_by_id(player_id).expect("Player not found");
            let input_flags: PlayerInputFlags = PlayerInputFlags::from_bits(input).expect("Invalid input");
            player.input = input_flags;
        }
        pub fn output(&self) -> ServerOutput {
            let response_vec: Vec<ControllerResponse> = self.players.iter().map(|player: &Player| {
                let mut response_object = ControllerResponse::new();
                response_object.set_position(player.position.to_buffer_point());
                response_object.set_cannon_position(player.calculate_cannon_position().to_buffer_point());
                response_object
            }).collect();

            let mut server_output = ServerOutput::new();
            let repeated_field = RepeatedField::from_vec(response_vec);
            server_output.set_responses(repeated_field);

            server_output
        }
        pub fn add_player(&mut self) -> i32 {
            self.id_count += 1;
            self.players.push(Player {
                id: self.id_count,
                position: ControllerPoint::random_point(self.height, self.width),
                cannon_angle: 0,
                input: PlayerInputFlags::noinput
            });
            self.id_count
        }
        pub fn drop_player(&mut self, player_id: i32) {
            let index = self.players.iter().position(|player| player.id == player_id).unwrap();
            self.players.remove(index);
        }
        fn get_player_by_id(&mut self, player_id: i32) -> Result<&mut Player, String> { // make this look nicer
            if let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) {
                Ok(player)
            } else {
                Err(format!("Can't find player with id: {}", player_id))
            }
        }
    }
}