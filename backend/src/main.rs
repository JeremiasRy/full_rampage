use std::borrow::Borrow;

use bitflags::bitflags;

bitflags! {
    struct Direction: u8 {
        const up = 0b0001;
        const right = 0b0010;
        const down = 0b0100;
        const left = 0b1000;
    }
}
#[derive(PartialEq)]
#[derive(Debug)]
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
    input: u8
}
#[derive(Debug)]
struct Player {
    id: i32,
    position: Point
}

impl Player {
    pub fn move_me(&mut self, dx: i32, dy:i32) {
        self.position.translate(dx, dy);
    }
}

struct GameController {
    height: u32,
    width: u32,
    players: Vec<Player>
}

impl GameController {
    pub fn player_input(&mut self, input: PlayerInput) {
        let player = self.get_player_by_id(input.player_id).expect("Player not found");
        let mut dx = 0;
        let mut dy = 0;
        let direction = Direction::from_bits(input.input).expect("Invalid input");
                
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
    }
    fn get_player_by_id(&mut self, player_id: i32) -> Result<&mut Player, String> {
        if let Some(player) = self.players.iter_mut().find(|player| player.id == player_id) {
            return Ok(player)
        } else {
            return Err(format!("Can't find player with id: {}", player_id))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_input() {
        let players: Vec<Player> = vec![
            Player {id: 1, position: Point{x: 10, y: 10}},
            Player {id: 2, position: Point{x: 20, y: 20}},
            Player {id: 3, position: Point{x: 30, y: 30}},
            Player {id: 4, position: Point{x: 40, y: 40}},
            Player {id: 5, position: Point{x: 50, y: 50}},
            Player {id: 6, position: Point{x: 60, y: 60}},
            Player {id: 7, position: Point{x: 70, y: 70}},
            Player {id: 8, position: Point{x: 80, y: 80}},
        ];

        let mut game_controller = GameController {
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
            let player_input = PlayerInput {
                player_id: (index + 1) as i32,
                input: element
            };
            game_controller.player_input(player_input);
        }
        assert_eq!(game_controller.players[0].position, Point {x: 10, y: 9}); // player moved up
        assert_eq!(game_controller.players[1].position, Point {x: 21, y: 20}); // player moved right
        assert_eq!(game_controller.players[2].position, Point {x: 30, y: 31}); // player moved down
        assert_eq!(game_controller.players[3].position, Point {x: 39, y: 40}); // player moved left
        assert_eq!(game_controller.players[4].position, Point {x: 51, y: 49}); // player moved up-right
        assert_eq!(game_controller.players[5].position, Point {x: 59, y: 59}); // player moved up-left
        assert_eq!(game_controller.players[6].position, Point {x: 71, y: 71}); // player moved down-right
        assert_eq!(game_controller.players[7].position, Point {x: 79, y: 81}); // player moved down-left

    }
}
