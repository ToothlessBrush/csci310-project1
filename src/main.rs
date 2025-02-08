use rand::prelude::*;
use std::fmt;

use std::clone::Clone;

pub mod my_thread;

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, PartialEq)]
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
        Game {
            grid: [[None; 3]; 3],
            current_player: Player::X,
        }
    }

    fn play_turn(&mut self, x: usize, y: usize) -> bool {
        if self.grid[x][y].is_none() {
            self.grid[x][y] = Some(self.current_player);
            // switch player
            if self.current_player == Player::X {
                self.current_player = Player::O;
            } else {
                self.current_player = Player::X;
            }
            return true;
        }
        false
    }

    fn check_win(&self) -> Option<Player> {
        let grid = self.grid;

        // check rows
        for i in 0..3 {
            if grid[i][0].is_some() && grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
                return grid[i][0];
            }
        }

        // check columns
        for i in 0..3 {
            if grid[0][i].is_some() && grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
                return grid[0][i];
            }
        }

        // check diagonals
        if grid[0][0].is_some() && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            return grid[0][0];
        }

        if grid[0][2].is_some() && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            return grid[0][2];
        }

        if grid
            .iter()
            .all(|row| row.iter().all(|&cell| cell.is_some()))
        {
            return Some(Player::Stalemate);
        }

        None // no winner
    }
}

// function will run when println! is called on game
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
    let mut rng = rand::thread_rng();

    let mut game = Game::new();
    while game.check_win().is_none() {
        let mut success = false;
        while !success {
            let x = rng.gen_range(0..3);
            let y = rng.gen_range(0..3);
            success = game.play_turn(x, y);
        }
    }
    println!("{}", game);

    let mut number = Arc::new(Mutex::new(5));

    let number1 = number.clone();

    let thread = thread::spawn(move || {
        if let Ok(mut lock) = number1.lock() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("hello, thread: {}", lock);
            *lock += 1;
        }
    });

    let number2 = number.clone();

    let thread2 = thread::spawn(move || {
        if let Ok(mut lock) = number2.lock() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("hello, thread: {}", lock);
        }
    });

    thread.join();
    thread2.join();
}
