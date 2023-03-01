use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};
use serde::{Deserialize, Serialize};

use crate::words::{generate_wordlist_from_game, get_random_letter, get_random_n_length_word};

enum GameDirections {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Game {
    grid: Vec<Vec<char>>,
    valid_words: Vec<String>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new(5)
    }
}

impl Game {
    /// Creates a new `GeneratedGame` of `size` x `size`
    pub fn new(size: usize) -> Self {
        fn calculate_valid_directions(
            grid: &Vec<Vec<Option<char>>>,
            (y, x): &(usize, usize),
        ) -> Vec<GameDirections> {
            let grid_length = grid.len();
            let mut output = Vec::new();

            if *y > 0 && grid[*y - 1][*x] == None {
                output.push(GameDirections::Up);
            }
            if *y > 0 && *x > 0 && grid[*y - 1][*x - 1] == None {
                output.push(GameDirections::UpLeft);
            }
            if *y > 0 && *x < grid_length - 1 && grid[*y - 1][*x + 1] == None {
                output.push(GameDirections::UpRight);
            }
            if *y < grid_length - 1 && grid[*y + 1][*x] == None {
                output.push(GameDirections::Down);
            }
            if *y < grid_length - 1 && *x > 0 && grid[*y + 1][*x - 1] == None {
                output.push(GameDirections::DownLeft);
            }
            if *y < grid_length - 1 && *x < grid_length - 1 && grid[*y + 1][*x + 1] == None {
                output.push(GameDirections::DownRight);
            }
            if *x > 0 && grid[*y][*x - 1] == None {
                output.push(GameDirections::Left);
            }
            if *x < grid_length - 1 && grid[*y][*x + 1] == None {
                output.push(GameDirections::Right);
            }
            output
        }

        let target_word_size = (2 * size..3 * size)
            .into_iter()
            .choose(&mut thread_rng())
            .unwrap();
        let target_word = get_random_n_length_word(target_word_size);

        let mut grid = 'outer: loop {
            let mut grid = vec![vec![None; size]; size];
            let start_point = (
                (0..size).into_iter().choose(&mut thread_rng()).unwrap(),
                (0..size).into_iter().choose(&mut thread_rng()).unwrap(),
            );
            let mut point = start_point;

            for character in target_word.chars() {
                grid[point.0][point.1] = Some(character);
                match calculate_valid_directions(&grid, &point).choose(&mut thread_rng()) {
                    None => continue 'outer,
                    Some(GameDirections::Up) => point = (point.0 - 1, point.1),
                    Some(GameDirections::UpLeft) => point = (point.0 - 1, point.1 - 1),
                    Some(GameDirections::UpRight) => point = (point.0 - 1, point.1 + 1),
                    Some(GameDirections::Down) => point = (point.0 + 1, point.1),
                    Some(GameDirections::DownLeft) => point = (point.0 + 1, point.1 - 1),
                    Some(GameDirections::DownRight) => point = (point.0 + 1, point.1 + 1),
                    Some(GameDirections::Left) => point = (point.0, point.1 - 1),
                    Some(GameDirections::Right) => point = (point.0, point.1 + 1),
                }
            }
            break grid;
        };

        for c in grid.iter_mut().flatten() {
            if *c == None {
                *c = Some(get_random_letter())
            }
        }

        let grid = grid
            .into_iter()
            .map(|r| r.into_iter().map(|c| c.unwrap()).collect())
            .collect();

        Game {
            valid_words: generate_wordlist_from_game(&grid),
            grid,
        }
    }

    pub fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }

    pub fn valid_words(&self) -> &Vec<String> {
        &self.valid_words
    }

    pub fn validate(&self, word: &str) -> bool {
        self.valid_words.binary_search(&word.to_string()).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_generation() {
        let _x = Game::new(5);
        let _y = Game::new(4);
    }

    #[test]
    fn wordlist_validation() {
        let game = Game {
            grid: vec![vec![]],
            valid_words: vec![
                "bar".to_string(),
                "baz".to_string(),
                "foo".to_string(),
                "qux".to_string(),
            ],
        };
        assert!(game.validate("foo"));
        assert!(game.validate("bar"));
        assert!(game.validate("baz"));
        assert!(game.validate("qux"));

        let game = Game::new(5);
        assert!(game.validate(&game.valid_words[2]));
        assert!(game.validate(&game.valid_words[game.valid_words().len() - 1]));
    }
}
