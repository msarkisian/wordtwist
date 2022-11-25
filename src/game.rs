use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use crate::words::{get_random_letter, get_random_n_length_word};

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

#[derive(Debug)]
pub struct GeneratedGame<const N: usize> {
    grid: [[char; N]; N],
    valid_words: Vec<String>,
}

impl<const N: usize> GeneratedGame<N> {
    pub fn new() -> Self {
        fn calculate_valid_directions<const N: usize>(
            grid: &[[char; N]; N],
            (y, x): &(usize, usize),
        ) -> Vec<GameDirections> {
            let mut output = Vec::new();

            if *y > 0 && grid[*y - 1][*x] == '0' {
                output.push(GameDirections::Up);
            }
            if *y > 0 && *x > 0 && grid[*y - 1][*x - 1] == '0' {
                output.push(GameDirections::UpLeft);
            }
            if *y > 0 && *x < N - 2 && grid[*y - 1][*x + 1] == '0' {
                output.push(GameDirections::UpRight);
            }
            if *y < N - 2 && grid[*y + 1][*x] == '0' {
                output.push(GameDirections::Down);
            }
            if *y < N - 2 && *x > 0 && grid[*y + 1][*x - 1] == '0' {
                output.push(GameDirections::DownLeft);
            }
            if *y < N - 2 && *x < N - 2 && grid[*y + 1][*x + 1] == '0' {
                output.push(GameDirections::DownRight);
            }
            if *x > 0 && grid[*y][*x - 1] == '0' {
                output.push(GameDirections::Left);
            }
            if *x < N - 2 && grid[*y][*x + 1] == '0' {
                output.push(GameDirections::Right);
            }
            output
        }

        let target_word_size = (2 * N..3 * N)
            .into_iter()
            .choose(&mut thread_rng())
            .unwrap();
        let target_word = get_random_n_length_word(target_word_size);
        println!("Target word: {}", target_word);

        let mut grid = 'outer: loop {
            let mut grid = [['0'; N]; N];
            let start_point = (
                (0..N).into_iter().choose(&mut thread_rng()).unwrap(),
                (0..N).into_iter().choose(&mut thread_rng()).unwrap(),
            );
            let mut point = start_point;

            for character in target_word.chars() {
                grid[point.0][point.1] = character;
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
            if *c == '0' {
                *c = get_random_letter()
            }
        }

        // TODO: generate valid words from board

        GeneratedGame {
            grid,
            valid_words: vec![String::from("todo")],
        }
    }

    pub fn grid(&self) -> &[[char; N]; N] {
        &self.grid
    }

    pub fn valid_words(&self) -> &Vec<String> {
        &self.valid_words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_generation() {
        let _x = GeneratedGame::<5>::new();
        let _y = GeneratedGame::<4>::new();
        println!("{:?}", GeneratedGame::<5>::new());
        println!("{:?}", GeneratedGame::<4>::new());
        panic!();
    }
}
