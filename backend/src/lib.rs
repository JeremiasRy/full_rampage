include!(concat!(env!("OUT_DIR"), "/messages.rs"));

pub mod gamelogic {
    use crate::{CannonEventResponse, PlayerResponse, Point, ServerOutput};
    use std::{collections::VecDeque, ops::Mul};
    use std::collections::hash_map::HashMap;
    use protobuf::RepeatedField;
    use rand::{thread_rng, Rng};

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
    struct Explosion {
        position: ControllerPoint,
        from_player_id: i32,
        size: i32,
        reached_max_size: bool
    }

    impl Explosion {
        pub fn new(from_player_id: i32, position: ControllerPoint) -> Self {
            Explosion {
                from_player_id,
                position,
                size: 1,
                reached_max_size: false
            }
        }
        pub fn tick(&mut self) {

            if !self.reached_max_size && self.size >= 100 {
                self.reached_max_size = true;
            }

            if self.reached_max_size {
                self.size -= 20;
                return;
            }
            self.size += 10;
        }
        pub fn check_for_hit(&mut self, player_pos: ControllerPoint) -> bool {
            let current_explosion_radius = self.size / 2;
            let radius_and_player_size = current_explosion_radius as f32 + PLAYER_SIZE;
            if (self.position.x - player_pos.x).abs() > radius_and_player_size && (self.position.y - player_pos.y).abs() > radius_and_player_size {
                return false
            }
            let closest_x = (self.position.x).max(player_pos.x).min(player_pos.x + PLAYER_SIZE);
            let closest_y = (self.position.y).max(player_pos.y).min(player_pos.y + PLAYER_SIZE);

            let distance_x = self.position.x - closest_x;
            let distance_y = self.position.y - closest_y;

            distance_x * distance_x + distance_y * distance_y < (self.size * self.size) as f32
        }
    }

    #[derive(Debug)]
    struct CannonShot {
        from_player_id: i32,
        distance_to_travel: f32,
        trajectory: VecDeque<ControllerPoint>,
        last_position: Option<ControllerPoint>,
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
        pub fn tick(&mut self) {
            self.position = self.trajectory.pop_front();

            match self.position {
                Some(pos) => {
                    self.last_position = Some(pos.clone());
                },
                _ => ()
            }
        }
        pub fn new(from_player_id: i32, from: ControllerPoint, angle: f32, power: i32) -> Self {
            let distance = MAX_CANNON_SHOT_LENGTH as f32 * (power as f32 / 100.0);
            let step_size = 30.0;
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
                last_position: None,
                position: None,
                from_player_id,
                distance_to_travel: trajectory.len() as f32,
                trajectory
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum PlayerStatus {
        Alive,
        Dead,
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
        cooldown: i32,
        status: PlayerStatus
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
                cooldown: 0,
                status: PlayerStatus::Alive
            }
        }
        pub fn die(&mut self) {
            self.status = PlayerStatus::Dead;
            self.cooldown = 240;
        }
        pub fn tick(&mut self) {

            if self.cooldown > 0 {
                self.cooldown -= 1;
                if self.cooldown <= 0 {
                    self.status = PlayerStatus::Alive;
                }
                return;
            }

            self.check_vertical();

            self.check_horizontal();

            self.check_angle();

            self.check_shooting();

            if self.is_moving() {
                self.check_wall_collision();
            }

            self.translate();
        }
        pub fn check_player_collision(&self, other: &Player) -> bool {
            !(self.position.x + PLAYER_SIZE <= other.position.x || 
            self.position.y + PLAYER_SIZE <= other.position.y || 
            other.position.x + PLAYER_SIZE <= self.position.x ||
            other.position.y + PLAYER_SIZE <= self.position.y) 
        }
        pub fn should_tick(&self) -> bool {
            self.cooldown > 0 || self.cannon_shot.is_some() || self.input > 0 || self.delta_x != 0.0 || self.delta_y != 0.0
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
            delta.abs() >= 15.0
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
                self.input -= PlayerInput::Fire as i32;
                self.is_loading_cannon = false;
                self.power_loaded = 0;
            }
        }

        fn check_wall_collision(&mut self) {
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
        handle_collisions: VecDeque<(i32, i32)>,
        players: HashMap<i32, Player>,
        cannon_shots: HashMap<i32, CannonShot>,
        explosions: HashMap<i32, Explosion>,
        internal_id_count: i32
    }

    impl GameController {
        pub fn new() -> GameController {
            GameController {
                internal_id_count: 0,
                height: BOUNDS_HEIGHT,
                width: BOUNDS_WIDTH,
                players: HashMap::<i32, Player>::new(),
                cannon_shots: HashMap::<i32, CannonShot>::new(),
                explosions: HashMap::<i32, Explosion>::new(),
                handle_collisions: VecDeque::<(i32, i32)>::new(),
            }
        }
        pub fn tick(&mut self) {
            let mut cannon_shot_ids_marked_for_remove = Vec::with_capacity(self.cannon_shots.len());
            let mut explosions_marked_for_remove = Vec::with_capacity(self.explosions.len());

            self.check_player_collisions();
            println!("We have {} collisions in tick", self.handle_collisions.len());
            while let Some(player_id_pair) = self.handle_collisions.pop_front() {
                let player_ids = vec![player_id_pair.0, player_id_pair.1];
                let mut players: Vec<&mut Player> = self.players
                    .iter_mut()
                    .filter(|(id, _)| player_ids.contains(id))
                    .map(|(_, player)| player)
                    .collect();

                if let [first, second] = &mut players[..] {
                    GameController::handle_collision(first, second);
                } else {
                    panic!("Failed to obtain two mutable references to players");
                }
            }

            if !self.cannon_shots.is_empty() {
                for (id, cannon_shot) in self.cannon_shots.iter_mut() {
                    cannon_shot.tick();
                    if let None = cannon_shot.position {
                        self.internal_id_count += 1;
                        cannon_shot_ids_marked_for_remove.push(*id);
                        self.explosions.insert(self.internal_id_count, Explosion::new(cannon_shot.from_player_id, cannon_shot.last_position.unwrap()));
                    }
                }
            }

            if !self.explosions.is_empty() {
                for (id, explosion) in self.explosions.iter_mut() {
                    explosion.tick();
                    if explosion.size <= 0 {
                        explosions_marked_for_remove.push(*id);
                        continue;
                    }
                    for (_, player) in self.players.iter_mut().filter(|(_, player)| explosion.check_for_hit(player.position)) {
                        player.die();
                    };
                }
            }

            for player in self.players.values_mut().filter(|player| player.should_tick()) {
                player.tick();
                if let Some(cannon_shot) = player.cannon_shot.take() {
                    self.internal_id_count += 1;
                    self.cannon_shots.insert(self.internal_id_count, cannon_shot);
                }
            }

            for id in cannon_shot_ids_marked_for_remove {
                self.cannon_shots.remove_entry(&id);
            }
            for id in explosions_marked_for_remove {
                self.explosions.remove_entry(&id);
            }
        }
        pub fn should_tick(&self) -> bool {
            self.cannon_shots.len() > 0 || self.players.iter().any(|(_, player)| player.should_tick()) || !self.explosions.is_empty()
        }
        pub fn player_input(&mut self, input: InputRequest) {
            if let Some(player) = self.players.get_mut(&input.get_player_id()) {
                player.input(input.get_input());
            };
        }
        pub fn add_player(&mut self, id:i32) {
            self.players.insert(id,Player::new(id, self.height, self.width));
        }
        pub fn drop_player(&mut self, player_id: i32) {
            self.players.remove_entry(&player_id);
        }
        pub fn output(&mut self) -> ServerOutput {
            let mut player_response_vec: Vec<PlayerResponse> = Vec::<PlayerResponse>::new();
            let mut cannon_shot_response_vec = Vec::<CannonEventResponse>::new();
            let mut exlosion_response_vec = Vec::<CannonEventResponse>::new();

            for player in self.players.values() {
                let mut player_response = PlayerResponse::new();
                player_response.set_position(player.position.to_buffer_point());
                player_response.set_cannon_position(player.get_cannon_position().to_buffer_point());
                player_response.set_dead(player.status == PlayerStatus::Dead);
                player_response_vec.push(player_response)
            }

            for explosion in self.explosions.values() {
                let mut explosion_response = CannonEventResponse::new();

                explosion_response.set_position(explosion.position.to_buffer_point());
                explosion_response.set_size(explosion.size);
                exlosion_response_vec.push(explosion_response);
            }

            for cannon_shot in self.cannon_shots.values() {
                if let Some(position) = cannon_shot.position {
                    let mut cannon_shot_response = CannonEventResponse::new();
                    cannon_shot_response.set_position(position.to_buffer_point());
                    cannon_shot_response.set_size(cannon_shot.count_size());

                    cannon_shot_response_vec.push(cannon_shot_response);
                } 
            }

            let mut server_output = ServerOutput::new();
            let players = RepeatedField::from_vec(player_response_vec);
            let cannon_shots = RepeatedField::from_vec(cannon_shot_response_vec);
            let explosions = RepeatedField::from_vec(exlosion_response_vec);
            server_output.set_players(players);
            server_output.set_shots(cannon_shots);
            server_output.set_explosions(explosions);
            server_output.set_field_type(crate::MessageType::frame);

            server_output
        }
        fn check_player_collisions(&mut self) {
            for player in self.players.values() {
                for other_player in self.players.values().filter(|other_player| player.id != other_player.id) {
                    if player.check_player_collision(other_player) {
                        if !self.handle_collisions.iter().any(|(first, second)| *first == player.id || *first == other_player.id ||*second == player.id || *second == other_player.id) {
                            self.handle_collisions.push_back((player.id, other_player.id));
                        }
                    }
                }
            }
        }
        fn handle_collision(first: &mut Player, second: &mut Player) {
            // lets handle
        }
    }
}