pub mod gamelogic {
    use std::f32::consts::PI;
    use bitflags::bitflags;
    const PLAYER_SIZE: i32 = 25;
    const CANNON_LENGTH: i32 = 25;

    bitflags! {
        struct Direction: u8 {
            const up = 0b000001;
            const right = 0b000010;
            const down = 0b000100;
            const left = 0b001000;
            const cannon_positive = 0b010000;
            const cannon_negative = 0b100000;
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32
    }

    impl Point {
        pub fn translate(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y += dy;
        }
    }
    #[derive(Debug)]
    struct PlayerInput {
        player_id: i32,
        input: u8,
    }
    #[derive(Debug)]
    struct Player {
        id: i32,
        position: Point,
        cannon_angle: i32,
    }

    #[derive(Debug)]
    struct ControllerPlayerOutput {
        position: Point,
        cannon_position: Point
    }

    impl Player {
        pub fn move_me(&mut self, dx: i32, dy:i32) {
            self.position.translate(dx, dy);
        }
        pub fn move_cannon(&mut self, da: i32) {
            self.cannon_angle = self.cannon_angle + da % 359;
        }
        fn calculate_cannon_position(&self) -> Point {
            let cannon_radians = self.cannon_angle as f32 * PI / 180f32;
            let (center_x, center_y) = (self.position.x + PLAYER_SIZE / 2, self.position.y + PLAYER_SIZE / 2);
            let dx = CANNON_LENGTH as f32 * cannon_radians.cos();
            let dy = CANNON_LENGTH as f32 * cannon_radians.sin();
            Point {
                x: dx.round() as i32 + center_x,
                y: dy.round() as i32 + center_y
            }
        }
    }

    struct GameController {
        height: u32,
        width: u32,
        players: Vec<Player>
    }

    impl GameController {
        pub fn player_input(&mut self, input: PlayerInput) {
            let player: &mut Player = self.get_player_by_id(input.player_id).expect("Player not found");
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;
            let mut da: i32 = 0;
            let direction: Direction = Direction::from_bits(input.input).expect("Invalid input");
                
            if direction.contains(Direction::up) {
                dy -= 1;
            }
            if direction.contains(Direction::down) {
                dy += 1;
            }
            if direction.contains(Direction::left) {
                dx -= 1;
            }
            if direction.contains(Direction::right) {
                dx += 1;
            }
            player.move_me(dx, dy);

            if direction.contains(Direction::cannon_negative) {
                da -= 1;
            }

            if direction.contains(Direction::cannon_positive) {
                da += 1;
            }

            if da != 0 {
                player.move_cannon(da);
            }
        }
        pub fn output(&self) -> Vec<ControllerPlayerOutput> {
            self.players.iter().map(|player: &Player| ControllerPlayerOutput {position: player.position, cannon_position: player.calculate_cannon_position()}).collect()
        }
        fn get_player_by_id(&mut self, player_id: i32) -> Result<&mut Player, String> { // make this look nicer
            if let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) {
                return Ok(player)
            } else {
                return Err(format!("Can't find player with id: {}", player_id))
            }
        }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_input() {
        let players: Vec<Player> = vec![
            Player {id: 1, position: Point{x: 10, y: 10}, cannon_angle: 180},
            Player {id: 2, position: Point{x: 20, y: 20}, cannon_angle: 0},
            Player {id: 3, position: Point{x: 30, y: 30}, cannon_angle: 0},
            Player {id: 4, position: Point{x: 40, y: 40}, cannon_angle: 0},
            Player {id: 5, position: Point{x: 50, y: 50}, cannon_angle: 0},
            Player {id: 6, position: Point{x: 60, y: 60}, cannon_angle: 0},
            Player {id: 7, position: Point{x: 70, y: 70}, cannon_angle: 0},
            Player {id: 8, position: Point{x: 80, y: 80}, cannon_angle: 180},
        ];

        let mut controller_output = GameController {
            height: 100,
            width: 100,
            players,
        };

        let inputs: [u8; 8] = [
            Direction::up.bits(), 
            Direction::right.bits(), 
            Direction::down.bits(), 
            Direction::left.bits(), 
            (Direction::up | Direction::right).bits(), 
            (Direction::up | Direction::left).bits(), 
            (Direction::down | Direction::right).bits(), 
            (Direction::down | Direction::left).bits()
        ];

        for (index, &element) in inputs.iter().enumerate() {
            let player_input: PlayerInput = PlayerInput {
                player_id: (index + 1) as i32,
                input: element,
            };
            controller_output.player_input(player_input);
        }
        let controller_output: Vec<ControllerPlayerOutput> = controller_output.output();
        println!("{:?}", controller_output);
        assert_eq!(controller_output[0].position, Point {x: 10, y: 9}); // player moved up
        assert_eq!(controller_output[1].position, Point {x: 21, y: 20}); // player moved right
        assert_eq!(controller_output[2].position, Point {x: 30, y: 31}); // player moved down
        assert_eq!(controller_output[3].position, Point {x: 39, y: 40}); // player moved left
        assert_eq!(controller_output[4].position, Point {x: 51, y: 49}); // player moved up-right
        assert_eq!(controller_output[5].position, Point {x: 59, y: 59}); // player moved up-left
        assert_eq!(controller_output[6].position, Point {x: 71, y: 71}); // player moved down-right
        assert_eq!(controller_output[7].position, Point {x: 79, y: 81}); // player moved down-left
    }
}
}