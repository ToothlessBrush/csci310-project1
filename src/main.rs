use rand::prelude::*;
use std::fmt;
use std::clone::Clone;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    X,
    O,
    Stalemate,
}

struct Game {
    grid: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        // Randomly choose a starting player.
        let mut rng = rand::thread_rng();
        let starting_player = if rng.gen_bool(0.5) {
            Player::X
        } else {
            Player::O
        };

        println!("Random starting player: {:?}", starting_player);

        Game {
            grid: [[None; 3]; 3],
            current_player: starting_player,
        }
    }

    /// Attempts to play a turn at (x, y). If the cell is free, mark it and switch the turn.
    fn play_turn(&mut self, x: usize, y: usize) -> bool {
        if self.grid[x][y].is_none() {
            self.grid[x][y] = Some(self.current_player);
            self.current_player = if self.current_player == Player::X {
                Player::O
            } else {
                Player::X
            };
            return true;
        }
        false
    }

    /// Check rows, columns, and diagonals for a winner.
    fn check_win(&self) -> Option<Player> {
        let grid = self.grid;

        // Check rows.
        for i in 0..3 {
            if grid[i][0].is_some() && grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
                return grid[i][0];
            }
        }

        // Check columns.
        for i in 0..3 {
            if grid[0][i].is_some() && grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
                return grid[0][i];
            }
        }

        // Check diagonals.
        if grid[0][0].is_some() && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            return grid[0][0];
        }
        if grid[0][2].is_some() && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            return grid[0][2];
        }

        // If all cells are filled, it's a stalemate.
        if grid.iter().all(|row| row.iter().all(|&cell| cell.is_some())) {
            return Some(Player::Stalemate);
        }

        None // No winner.
    }
}

// Function will run when println! is called on game
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.grid {
            for col in row {
                match col {
                    Some(Player::X) => write!(f, " X ")?,
                    Some(Player::O) => write!(f, " O ")?,
                    None => write!(f, " . ")?,
                    _ => {}
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // Wrap the game in an Arc and Mutex to share among threads.
    let game = Arc::new(Mutex::new(Game::new()));

    // Spawn thread for Player X.
    let game_for_x = Arc::clone(&game);
    let handle_x = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let mut game = game_for_x.lock().unwrap();

            // If game over, print final board and break.
            if let Some(winner) = game.check_win() {
                println!("Final board:\n{}", *game);
                if winner == Player::Stalemate {
                    println!("Game over: Stalemate!");
                } else {
                    println!("Game over: {:?} wins!", winner);
                }
                break;
            }

            // Only proceed if it's Player X's turn.
            if game.current_player != Player::X {
                drop(game); // release lock before sleeping
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            // Try a random move until successful.
            let mut success = false;
            while !success {
                let x = rng.gen_range(0..3);
                let y = rng.gen_range(0..3);
                success = game.play_turn(x, y);
            }
            println!("(X moved) Board now:\n{}", *game);
        }
    });

    // Spawn thread for Player O.
    let game_for_o = Arc::clone(&game);
    let handle_o = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let mut game = game_for_o.lock().unwrap();

            // Check for game over without binding the winner.
            if game.check_win().is_some() {
                break;
            }

            if game.current_player != Player::O {
                drop(game);
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            let mut success = false;
            while !success {
                let x = rng.gen_range(0..3);
                let y = rng.gen_range(0..3);
                success = game.play_turn(x, y);
            }
            println!("(O moved) Board now:\n{}", *game);
        }
    });

    // Wait for both threads to finish.
    handle_x.join().unwrap();
    handle_o.join().unwrap();
}
