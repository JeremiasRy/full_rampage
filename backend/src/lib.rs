include!(concat!(env!("OUT_DIR"), "/messages.rs"));

pub mod gamelogic {
    use std::{borrow::BorrowMut, collections::VecDeque};
    use std::collections::hash_map::HashMap;
    use bitflags::bitflags;
    use protobuf::RepeatedField;
    use rand::{thread_rng, Rng};
    use crate::{CannonShotResponse, PlayerResponse, Point, ServerOutput};
    const PLAYER_SIZE: f32 = 25.0;
    const CANNON_LENGTH: f32 = 25.0;
    const MAX_CANNON_SHOT_LENGTH: i32 = 300;

    type InputRequest = crate::InputRequest;

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
            const load_cannon = 0b1000000;
            const fire = 0b10000000;
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct ControllerPoint {
        x: f32,
        y: f32
    }

    impl ControllerPoint {
        pub fn translate(&mut self, dx: f32, dy: f32) {
            self.x += dx;
            self.y += dy;
        }
        pub fn random_point(height_bounds: i32, width_bounds: i32) -> ControllerPoint {
            ControllerPoint {
                x: thread_rng().gen_range(0..=width_bounds) as f32,
                y: thread_rng().gen_range(0..=height_bounds) as f32
            }
        }
        pub fn to_buffer_point(&self) -> Point {
            let mut point = Point::new();
            point.set_x(self.x as i32);
            point.set_y(self.y as i32);

            point
        }
    }

    #[derive(Debug)]
    struct CannonShot {
        distance_to_travel: f32,
        trajectory: VecDeque<ControllerPoint>,
    }

    struct CannonShotPosition {
        size: i32,
        position: Option<ControllerPoint>
    }

    impl CannonShot {
        fn count_size(&self) -> i32 {
            let current_point_in_trajectory = self.distance_to_travel - (self.trajectory.len() as f32);
            let halfway_point = self.distance_to_travel / 2.0;
            let over_half = current_point_in_trajectory > halfway_point;
            let mut result = (100.0 - ((halfway_point - current_point_in_trajectory) / halfway_point) * 100.0) as i32;

            if over_half {
                result = 100 - (result - 100);
            }

            result
        }
        pub fn get_pos(&mut self) -> CannonShotPosition {
            CannonShotPosition {
                size: self.count_size(),
                position: self.trajectory.pop_front(),
            }
        }
        pub fn new(from: ControllerPoint, angle: f32, power: i32) -> CannonShot {
            let distance = MAX_CANNON_SHOT_LENGTH as f32 * (power as f32 / 100.0);
            let step_size = 10.0;
            let radians = angle.to_radians();
            
            let dx = step_size * radians.cos();
            let dy = step_size * radians.sin();

            let mut trajectory = VecDeque::<ControllerPoint>::new();
            
            let mut current_x = from.x;
            let mut current_y = from.y;
            for _ in 0..distance.ceil() as i32 {
                trajectory.push_back(ControllerPoint { x: current_x, y: current_y });
                current_x += dx;
                current_y += dy;
            }
            CannonShot {
                distance_to_travel: trajectory.len() as f32,
                trajectory
            }
        }
    }

    #[derive(Debug)]
    struct Player {
        position: ControllerPoint,
        cannon_angle: f32,
        is_loading_cannon: bool,
        power_loaded: i32,
        cannon_shot: Option<CannonShot>,
        input: PlayerInputFlags,
        delta_x: f32,
        delta_y: f32
    }

    impl Player {
        pub fn new(max_height: i32, max_width: i32) -> Player {
            Player {
                position: ControllerPoint::random_point(max_height, max_width),
                cannon_angle: 0.0,
                input: PlayerInputFlags::noinput,
                delta_x: 0.0,
                delta_y: 0.0,
                is_loading_cannon: false,
                cannon_shot: None,
                power_loaded: 0,
            }
        }
        pub fn should_tick(&self) -> bool {
            self.cannon_shot.is_some() || self.input.bits() > 0 || self.delta_x != 0.0 || self.delta_y != 0.0
        }
        pub fn input(&mut self, input:PlayerInputFlags) {
            self.input = input
        }
        fn check_vertical(&mut self) {
            if self.input.contains(PlayerInputFlags::up) {
                self.delta_y -= 1.0;
            } else if !self.input.contains(PlayerInputFlags::down) && self.delta_y < 0.0 {
                self.delta_y += 1.0
            }
            if self.input.contains(PlayerInputFlags::down) {
                self.delta_y += 1.0;
            } else if !self.input.contains(PlayerInputFlags::up) && self.delta_y > 0.0 {
                self.delta_y -= 1.0;
            }
        }

        fn check_horizontal(&mut self) {
            if self.input.contains(PlayerInputFlags::left) {
                self.delta_x -= 1.0;
            } else if !self.input.contains(PlayerInputFlags::right) && self.delta_x < 0.0 {
                self.delta_x += 1.0
            }
            if self.input.contains(PlayerInputFlags::right) {
                self.delta_x += 1.0;
            } else if !self.input.contains(PlayerInputFlags::up) && self.delta_x > 0.0 {
                self.delta_x -= 1.0;
            }
        }
        pub fn tick(&mut self) {
            let mut da: f32 = 0.0;

            self.check_vertical();

            self.check_horizontal();

            if self.input.contains(PlayerInputFlags::cannon_positive) {
                da += 1.0;
            }
            if self.input.contains(PlayerInputFlags::cannon_negative) {
                da -= 1.0;
            }
            if self.input.contains(PlayerInputFlags::load_cannon) && !self.is_loading_cannon {
                self.is_loading_cannon = true;
            }
            
            if !self.input.contains(PlayerInputFlags::load_cannon) && self.is_loading_cannon {
                self.cannon_shot = Some(CannonShot::new(self.calculate_cannon_position(), self.cannon_angle, self.power_loaded));
                self.is_loading_cannon = false;
                self.power_loaded = 0;
            }

            self.position.translate(self.delta_x, self.delta_y);
            let mut new_angle = (self.cannon_angle + da) % 359.0;

            if new_angle < 0.0 {
                new_angle += 360.0;
            }
            if self.is_loading_cannon && self.power_loaded < 100 {
                self.power_loaded += 1;
            }
            self.cannon_angle = new_angle;
        }

        fn calculate_cannon_position(&self) -> ControllerPoint {
            let cannon_radians = (self.cannon_angle as f32).to_radians();
            let (center_x, center_y) = (self.position.x + PLAYER_SIZE / 2.0, self.position.y + PLAYER_SIZE / 2.0);
            let dx = CANNON_LENGTH as f32 * cannon_radians.cos();
            let dy = CANNON_LENGTH as f32 * cannon_radians.sin();
            ControllerPoint {
                x: dx + center_x,
                y: dy + center_y
            }
        }

    }

    pub struct GameController {
        height: i32,
        width: i32,
        players: HashMap<i32, Player>,
        cannon_shots: HashMap<i32, CannonShot>,
        id_count: i32
    }

    impl GameController {
        pub fn new() -> GameController {
            GameController {
                id_count: 0,
                height: 800,
                width: 1200,
                players: HashMap::<i32, Player>::new(),
                cannon_shots: HashMap::<i32, CannonShot>::new()
            }
        }
        pub fn tick(&mut self) {
            for (_, player) in self.players.iter_mut().filter(|(_, player)| player.should_tick()) {
                player.tick();

                if let Some(cannon_shot) = player.cannon_shot.take() {
                    self.id_count += 1;
                    self.cannon_shots.insert(self.id_count, cannon_shot);
                }
            }
        }
        pub fn should_tick(&self) -> bool {
            self.cannon_shots.len() > 0 || self.players.iter().any(|(_, player)| player.should_tick())
        }
        pub fn player_input(&mut self, input: InputRequest) {
            let player: &mut Player = self.get_player_by_id(input.player_id);
            let input_flags: PlayerInputFlags = PlayerInputFlags::from_bits(input.input.try_into().unwrap()).expect("Invalid input");
            player.input(input_flags);
        }
        pub fn output(&mut self) -> ServerOutput {
            let player_response_vec: Vec<PlayerResponse> = self.players.values_mut().map(|player| {
                let mut response_object = PlayerResponse::new();
                response_object.set_position(player.position.to_buffer_point());
                response_object.set_cannon_position(player.calculate_cannon_position().to_buffer_point());
                response_object
            }).collect();

            let mut cannon_shot_response_vec = Vec::<CannonShotResponse>::new();
            let mut marked_for_remove = Vec::<i32>::new();

            for (id, cannon_shot) in self.cannon_shots.iter_mut() {
                let cannon_shot_state = cannon_shot.get_pos();

                if let Some(position) = cannon_shot_state.position {
                    let mut cannon_shot_response = CannonShotResponse::new();
                    cannon_shot_response.set_position(position.to_buffer_point());
                    cannon_shot_response.set_size(cannon_shot_state.size);

                    cannon_shot_response_vec.push(cannon_shot_response);
                } else {
                    println!("Explosions!!");
                    marked_for_remove.push(*id);
                }
            }

            for id in marked_for_remove {
                self.cannon_shots.remove_entry(&id);
            }

            let mut server_output = ServerOutput::new();
            let players = RepeatedField::from_vec(player_response_vec);
            let cannon_shots = RepeatedField::from_vec(cannon_shot_response_vec);
            server_output.set_players(players);
            server_output.set_shots(cannon_shots);
            server_output.set_field_type(crate::MessageType::frame);

            server_output
        }
        pub fn add_player(&mut self) -> i32 {
            self.id_count += 1;
            self.players.insert(self.id_count,Player::new(self.height, self.width));
            self.id_count
        }
        pub fn drop_player(&mut self, player_id: i32) {
            self.players.remove_entry(&player_id);
        }
        fn get_player_by_id(&mut self, player_id: i32) -> &mut Player {
            self.players.get_mut(&player_id).unwrap()
        }
    }
}