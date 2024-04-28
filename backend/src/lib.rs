include!(concat!(env!("OUT_DIR"), "/messages.rs"));

pub mod gamelogic {
    use std::collections::VecDeque;
    use std::collections::hash_map::HashMap;
    use protobuf::RepeatedField;
    use rand::{thread_rng, Rng};
    use crate::{CannonShotResponse, PlayerResponse, Point, ServerOutput};
    const PLAYER_SIZE: f32 = 25.0;
    const CANNON_LENGTH: f32 = 25.0;
    const MAX_CANNON_SHOT_LENGTH: i32 = 300;
    const BOUNDS_HEIGHT: i32 = 800;
    const BOUNDS_WIDTH: i32 = 1200;

    type InputRequest = crate::InputRequest;

    enum PlayerInput {
        NoInput,
        Up = 1,
        Right = 1 << 1,
        Down = 1 << 2,
        Left = 1 << 3,
        AimPositive = 1 << 4,
        AimNegative = 1 << 5,
        LoadCannon = 1 << 6,
        Fire = 1 << 7
    }
    trait BitFlag {
        fn contains(&self, player_input:PlayerInput) -> bool;
    }
    impl BitFlag for i32 {
        fn contains(&self, player_input:PlayerInput) -> bool {
            let integer_representation = player_input as i32;

            self & integer_representation == integer_representation
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
        from_player_id: i32,
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

            if result < 20 {
                return 20
            }

            result
        }
        pub fn get_pos(&mut self) -> CannonShotPosition {
            CannonShotPosition {
                size: self.count_size(),
                position: self.trajectory.pop_front(),
            }
        }
        pub fn new(from_player_id: i32, from: ControllerPoint, angle: f32, power: i32) -> Self {
            let distance = MAX_CANNON_SHOT_LENGTH as f32 * (power as f32 / 100.0);
            let step_size = 20.0;
            let radians = angle.to_radians();
            
            let mut dx = step_size * radians.cos();
            let mut dy = step_size * radians.sin();

            let mut trajectory = VecDeque::<ControllerPoint>::new();
            
            let mut current_x = from.x;
            let mut current_y = from.y;
            for _ in 0..distance.ceil() as i32 {
                trajectory.push_back(ControllerPoint { x: current_x, y: current_y });
                current_x += dx;
                current_y += dy;

                if current_x < 0.0 || current_x > BOUNDS_WIDTH as f32 {
                    if dx < 0.0 {
                        dx = 0.0 + dx.abs();
                    } else if dx > 0.0 {
                        dx = 0.0 - dx;
                    }
                }

                if current_y < 0.0 || current_y > BOUNDS_HEIGHT as f32 {
                    if dy < 0.0 {
                        dy = 0.0 + dy.abs();
                    } else if dy > 0.0 {
                        dy = 0.0 - dy;
                    }
                }
            }
            CannonShot {
                from_player_id,
                distance_to_travel: trajectory.len() as f32,
                trajectory
            }
        }
    }

    #[derive(Debug)]
    struct Player {
        id: i32,
        position: ControllerPoint,
        cannon_angle: f32,
        is_loading_cannon: bool,
        power_loaded: i32,
        cannon_shot: Option<CannonShot>,
        input: i32,
        delta_x: f32,
        delta_y: f32,
        delta_a: f32,
    }

    impl Player {
        pub fn new(id: i32, max_height: i32, max_width: i32) -> Self {
            Player {
                id,
                position: ControllerPoint::random_point(max_height - PLAYER_SIZE as i32, max_width - PLAYER_SIZE as i32),
                cannon_angle: 0.0,
                input: PlayerInput::NoInput as i32,
                delta_x: 0.0,
                delta_y: 0.0,
                delta_a: 0.0,
                is_loading_cannon: false,
                cannon_shot: None,
                power_loaded: 0,
            }
        }
        pub fn tick(&mut self) {
            self.check_vertical();

            self.check_horizontal();

            self.check_angle();

            self.check_shooting();

            if self.is_moving() {
                self.check_collision();
            }

            self.translate();
        }
        pub fn should_tick(&self) -> bool {
            self.cannon_shot.is_some() || self.input > 0 || self.delta_x != 0.0 || self.delta_y != 0.0
        }
        pub fn input(&mut self, input:i32) {
            self.input = input
        }

        fn translate(&mut self) {
            let mut new_angle = (self.cannon_angle + self.delta_a) % 359.0;

            if new_angle < 0.0 {
                new_angle += 360.0;
            }
            if self.is_loading_cannon && self.power_loaded < 100 {
                self.power_loaded += 1;
            }
            self.cannon_angle = new_angle;
            self.position.translate(self.delta_x, self.delta_y);
        }

        fn is_moving(&self) -> bool {
            self.delta_x != 0.0 || self.delta_y != 0.0
        }

        fn is_at_top_speed(delta: f32) -> bool {
            delta.abs() >= 10.0
        }

        fn check_vertical(&mut self) {
            let is_at_top_speed = Player::is_at_top_speed(self.delta_y);
            if self.input.contains(PlayerInput::Up) && !is_at_top_speed {
                self.delta_y -= 1.0;
            } else if self.delta_y < 0.0 { // brakes
                self.delta_y += 1.0
            }
            if self.input.contains(PlayerInput::Down) && !is_at_top_speed {
                self.delta_y += 1.0;
            } else if self.delta_y > 0.0 { // brakes
                self.delta_y -= 1.0;
            }
        }

        fn check_horizontal(&mut self) {
            let is_at_top_speed = Player::is_at_top_speed(self.delta_x);
            if self.input.contains(PlayerInput::Left) && !is_at_top_speed {
                self.delta_x -= 1.0
            } else if self.delta_x < 0.0 { //brakes
                self.delta_x += 1.0;
            }
            if self.input.contains(PlayerInput::Right) && !is_at_top_speed {
                self.delta_x += 1.0;
            } else if self.delta_x > 0.0 { //brakes
                self.delta_x -= 1.0;
            }
        }

        fn check_angle(&mut self) {
            let is_at_top_speed = Player::is_at_top_speed(self.delta_a);
            if self.input.contains(PlayerInput::AimPositive) && !is_at_top_speed {
                self.delta_a += 1.0;
            } else if self.delta_a > 0.0 {
                self.delta_a = 0.0;
            }

            if self.input.contains(PlayerInput::AimNegative) && !is_at_top_speed {
                self.delta_a -= 1.0;
            } else if self.delta_a < 0.0 {
                self.delta_a = 0.0;
            }
        }

        fn check_shooting(&mut self) {
            if self.input.contains(PlayerInput::LoadCannon) && !self.is_loading_cannon {
                self.is_loading_cannon = true;
                return;
            }
            if self.input.contains(PlayerInput::Fire) && self.is_loading_cannon {
                self.cannon_shot = Some(CannonShot::new( self.id, self.get_cannon_position(), self.cannon_angle, self.power_loaded));
                self.is_loading_cannon = false;
                self.power_loaded = 0;
            }
        }

        fn check_collision(&mut self) {
            let horizontal_check = (self.position.x + self.delta_x) as i32;
            let vertical_check = (self.position.y + self.delta_y) as i32;

            if vertical_check < 0 || vertical_check + (PLAYER_SIZE as i32) > BOUNDS_HEIGHT {
                if self.delta_y < 0.0 {
                    self.delta_y = 0.0 + self.delta_y.abs()
                } else if self.delta_y > 0.0 {
                    self.delta_y = 0.0 - self.delta_y
                }
            }

            if horizontal_check < 0 || horizontal_check + (PLAYER_SIZE as i32) > BOUNDS_WIDTH {

                if self.delta_x < 0.0 {
                    self.delta_x = 0.0 + self.delta_x.abs();
                } else if self.delta_x > 0.0 {
                    self.delta_x = 0.0 - self.delta_x;
                }
            }
        }

        fn get_cannon_position(&self) -> ControllerPoint {
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
                height: BOUNDS_HEIGHT,
                width: BOUNDS_WIDTH,
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
            if let Some(player) = self.players.get_mut(&input.get_player_id()) {
                player.input(input.get_input());
            };
        }
        pub fn output(&mut self) -> ServerOutput {
            let player_response_vec: Vec<PlayerResponse> = self.players.values_mut().map(|player| {
                let mut response_object = PlayerResponse::new();
                response_object.set_position(player.position.to_buffer_point());
                response_object.set_cannon_position(player.get_cannon_position().to_buffer_point());
                response_object
            }).collect();

            let mut cannon_shot_response_vec = Vec::<CannonShotResponse>::new();
            let mut cannon_shot_ids_marked_for_remove = Vec::<i32>::new();

            for (id, cannon_shot) in self.cannon_shots.iter_mut() {
                let cannon_shot_state = cannon_shot.get_pos();

                if let Some(position) = cannon_shot_state.position {
                    let mut cannon_shot_response = CannonShotResponse::new();
                    cannon_shot_response.set_position(position.to_buffer_point());
                    cannon_shot_response.set_size(cannon_shot_state.size);

                    cannon_shot_response_vec.push(cannon_shot_response);
                } else {
                    println!("Explosions!!");
                    cannon_shot_ids_marked_for_remove.push(*id);
                }
            }

            for id in cannon_shot_ids_marked_for_remove {
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
            self.players.insert(self.id_count,Player::new(self.id_count, self.height, self.width));
            self.id_count
        }
        pub fn drop_player(&mut self, player_id: i32) {
            self.players.remove_entry(&player_id);
        }
    }
}