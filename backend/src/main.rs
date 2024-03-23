use bitflags::bitflags;

bitflags! {
    struct Direction: u8 {
        const up = 0b0001;
        const right = 0b0010;
        const down = 0b0100;
        const left = 0b1000;
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
    x: i32,
    y: i32
}

impl Player {
    pub fn move_me(&mut self, dx: i32, dy:i32) {
        self.x += dx;
        self.y += dy;
    }
}

struct GameController {
    height: u32,
    width: u32,
    players: Vec<Player>
}

impl GameController {
    pub fn player_input(&mut self, input: PlayerInput) {
        let player: Result<&mut Player, String> = self.get_player_by_id(input.player_id);
        match player {
            Ok(player) => {
                let mut dx = 0;
                let mut dy = 0;
                
                if input.input & Direction::up.bits() != 0 {
                    dy -= 1;
                }
                if input.input & Direction::down.bits() != 0 {
                    dy += 1;
                }
                if input.input & Direction::left.bits() != 0 {
                    dx -= 1;
                }
                if input.input & Direction::right.bits() != 0 {
                    dx += 1;
                }
                player.move_me(dx, dy);
            },
            Err(error) => panic!("{error}")
        }
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
            Player {id: 1, x: 10, y: 10},
            Player {id: 2, x: 20, y: 20},
            Player {id: 3, x: 30, y: 30},
            Player {id: 4, x: 40, y: 40},
            Player {id: 5, x: 50, y: 50},
            Player {id: 6, x: 60, y: 60},
            Player {id: 7, x: 70, y: 70},
            Player {id: 8, x: 80, y: 80},
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
        assert_eq!(game_controller.players[0].y, 9); // player moved up
        assert_eq!(game_controller.players[1].x, 21); // player moved right
        assert_eq!(game_controller.players[2].y, 31); // player moved down
        assert_eq!(game_controller.players[3].x, 39); // player moved left
        assert_eq!([game_controller.players[4].x, game_controller.players[4].y], [51, 49]); // player moved up-right
        assert_eq!([game_controller.players[5].x, game_controller.players[5].y], [59, 59]); // player moved up-left
        assert_eq!([game_controller.players[6].x, game_controller.players[6].y], [71, 71]); // player moved down-right
        assert_eq!([game_controller.players[7].x, game_controller.players[7].y], [79, 81]); // player moved down-left

    }
}
