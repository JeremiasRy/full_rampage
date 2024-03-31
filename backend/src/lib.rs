pub mod gamelogic {
    use bitflags::bitflags;
    use rand::{thread_rng, Rng};
    use serde::{Serialize, Deserialize};
    const PLAYER_SIZE: i32 = 25;
    const CANNON_LENGTH: i32 = 25;

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
    struct Point {
        x: i32,
        y: i32
    }

    impl Point {
        pub fn translate(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y += dy;
        }
        pub fn random_point(height_bounds: i32, width_bounds: i32) -> Point {
            Point {
                x: thread_rng().gen_range(0..=width_bounds),
                y: thread_rng().gen_range(0..=height_bounds)
            }
        }
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PlayerInputRequest {
        player_id: i32,
        input: u8,
    }

    #[derive(Debug, Serialize)]
    pub struct ControllerResponse {
        position: Point,
        cannon_position: Point
    }

    #[derive(Debug)]
    struct Player {
        id: i32,
        position: Point,
        cannon_angle: i32,
        input: PlayerInputFlags,
    }

    impl Player {
        pub fn has_input(&self) -> bool {
            self.id > 0
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

        fn calculate_cannon_position(&self) -> Point {
            let cannon_radians = (self.cannon_angle as f32).to_radians();
            let (center_x, center_y) = (self.position.x + PLAYER_SIZE / 2, self.position.y + PLAYER_SIZE / 2);
            let dx = CANNON_LENGTH as f32 * cannon_radians.cos();
            let dy = CANNON_LENGTH as f32 * cannon_radians.sin();
            Point {
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
        pub fn player_input(&mut self, input: PlayerInputRequest) {
            let player: &mut Player = self.get_player_by_id(input.player_id).expect("Player not found");
            let input_flags: PlayerInputFlags = PlayerInputFlags::from_bits(input.input).expect("Invalid input");
            player.input = input_flags;
        }
        pub fn output(&self) -> Vec<ControllerResponse> {
            self.players.iter().map(|player: &Player| ControllerResponse {position: player.position, cannon_position: player.calculate_cannon_position()}).collect()
        }
        pub fn add_player(&mut self) -> i32 {
            self.players.push(Player {
                id: self.id_count,
                position: Point::random_point(self.height, self.width),
                cannon_angle: 0,
                input: PlayerInputFlags::noinput
            });
            self.id_count += 1;
            self.id_count
        }
        fn get_player_by_id(&mut self, player_id: i32) -> Result<&mut Player, String> { // make this look nicer
            if let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) {
                Ok(player)
            } else {
                Err(format!("Can't find player with id: {}", player_id))
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_movement() {
        let players: Vec<Player> = vec![
            Player {id: 1, position: Point{x: 10, y: 10}, cannon_angle: 180, input: PlayerInputFlags::noinput},
            Player {id: 2, position: Point{x: 20, y: 20}, cannon_angle: 0, input: PlayerInputFlags::noinput},
            Player {id: 3, position: Point{x: 30, y: 30}, cannon_angle: 0, input: PlayerInputFlags::noinput},
            Player {id: 4, position: Point{x: 40, y: 40}, cannon_angle: 0, input: PlayerInputFlags::noinput},
            Player {id: 5, position: Point{x: 50, y: 50}, cannon_angle: 0, input: PlayerInputFlags::noinput},
            Player {id: 6, position: Point{x: 60, y: 60}, cannon_angle: 0, input: PlayerInputFlags::noinput},
            Player {id: 7, position: Point{x: 70, y: 70}, cannon_angle: 0, input: PlayerInputFlags::noinput},
            Player {id: 8, position: Point{x: 80, y: 80}, cannon_angle: 180, input: PlayerInputFlags::noinput},
        ];

        let mut controller = GameController {
            height: 100,
            width: 100,
            players,
            id_count: 0,
        };

        let inputs: [u8; 8] = [
            PlayerInputFlags::up.bits(), 
            PlayerInputFlags::right.bits(), 
            PlayerInputFlags::down.bits(), 
            PlayerInputFlags::left.bits(), 
            (PlayerInputFlags::up | PlayerInputFlags::right).bits(), 
            (PlayerInputFlags::up | PlayerInputFlags::left).bits(), 
            (PlayerInputFlags::down | PlayerInputFlags::right).bits(), 
            (PlayerInputFlags::down | PlayerInputFlags::left).bits()
        ];

        assert_eq!(controller.should_tick(), false);

        for (index, &input) in inputs.iter().enumerate() {
            let player_input: PlayerInputRequest = PlayerInputRequest {
                player_id: (index + 1) as i32,
                input,
            };
            controller.player_input(player_input);
        }
        assert_eq!(controller.should_tick(), true);
        controller.tick();

        let controller_output: Vec<ControllerResponse> = controller.output();
        assert_eq!(controller_output[0].position, Point {x: 10, y: 9}); // player moved up
        assert_eq!(controller_output[1].position, Point {x: 21, y: 20}); // player moved right
        assert_eq!(controller_output[2].position, Point {x: 30, y: 31}); // player moved down
        assert_eq!(controller_output[3].position, Point {x: 39, y: 40}); // player moved left
        assert_eq!(controller_output[4].position, Point {x: 51, y: 49}); // player moved up-right
        assert_eq!(controller_output[5].position, Point {x: 59, y: 59}); // player moved up-left
        assert_eq!(controller_output[6].position, Point {x: 71, y: 71}); // player moved down-right
        assert_eq!(controller_output[7].position, Point {x: 79, y: 81}); // player moved down-left
    }

    #[test]
    fn test_player_cannon_movement() {
        let players: Vec<Player> = vec![
            Player {id: 1, position: Point{x: 10, y: 10}, cannon_angle: 0, input: PlayerInputFlags::noinput},
            Player {id: 2, position: Point{x: 20, y: 20}, cannon_angle: 0, input: PlayerInputFlags::noinput},
        ];

        let mut controller: GameController = GameController {
            height: 100,
            width: 100,
            players,
            id_count: 0
        };

        let inputs: [u8; 2] = [
            PlayerInputFlags::cannon_negative.bits(), 
            PlayerInputFlags::cannon_positive.bits(), 
        ];

        assert_eq!(controller.should_tick(), false);
        println!("{:?}", controller.output());

        for (index, &input) in inputs.iter().enumerate() {
            let player_input: PlayerInputRequest = PlayerInputRequest {
                player_id: (index + 1) as i32,
                input,
            };
            controller.player_input(player_input);
        }
        assert_eq!(controller.should_tick(), true);
        controller.tick();
        controller.tick();
        controller.tick();
        controller.tick();
        controller.tick();

        assert_eq!(controller.players[0].cannon_angle, 355);
        assert_eq!(controller.players[1].cannon_angle, 5);

        let controller_output: Vec<ControllerResponse> = controller.output();
        println!("{:?}", controller_output[0].cannon_position.y);
        assert!(controller_output[0].cannon_position.y < 22);
        assert!(controller_output[1].cannon_position.y > 32);
    }
}
}