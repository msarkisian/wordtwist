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

            if *y > 0 && grid[*x][*y - 1] == '0' {
                output.push(GameDirections::Up);
            }
            if *y > 0 && *x > 0 && grid[*x - 1][*y - 1] == '0' {
                output.push(GameDirections::UpLeft);
            }
            if *y > 0 && *x < N - 2 && grid[*x + 1][*y - 1] == '0' {
                output.push(GameDirections::UpRight);
            }
            if *y < N - 2 && grid[*x][*y + 1] == '0' {
                output.push(GameDirections::Down);
            }
            if *y < N - 2 && *x > 0 && grid[*x - 1][*y + 1] == '0' {
                output.push(GameDirections::DownLeft);
            }
            if *y < N - 2 && *x < N - 2 && grid[*x + 1][*y + 1] == '0' {
                output.push(GameDirections::DownRight);
            }
            if *x > 0 && grid[*x - 1][*y] == '0' {
                output.push(GameDirections::Left);
            }
            if *x < N - 2 && grid[*x + 1][*y] == '0' {
                output.push(GameDirections::Right);
            }
            output
        }

        let target_word_size = (2 * N..3 * N)
            .into_iter()
            .choose(&mut thread_rng())
            .unwrap();
        let target_word = get_random_n_length_word(target_word_size);

        let target_word_grid = loop {
            let mut grid = [['0'; N]; N];
            let start_point = (
                (0..N).into_iter().choose(&mut thread_rng()).unwrap(),
                (0..N).into_iter().choose(&mut thread_rng()).unwrap(),
            );
            let mut point = start_point;

            for character in target_word.chars() {
                grid[point.0][point.1] = character;
                match calculate_valid_directions(&grid, &point)
                    .choose(&mut thread_rng())
                    .unwrap()
                    // Will panic if it gets boxed in
                    // TODO: FIX
                {
                    GameDirections::Up => point = (point.0, point.1 - 1),
                    GameDirections::UpLeft => point = (point.0 - 1, point.1 - 1),
                    GameDirections::UpRight => point = (point.0 + 1, point.1 - 1),
                    GameDirections::Down => point = (point.0, point.1 + 1),
                    GameDirections::DownLeft => point = (point.0 - 1, point.1 + 1),
                    GameDirections::DownRight => point = (point.0 + 1, point.1 + 1),
                    GameDirections::Left => point = (point.0 - 1, point.1),
                    GameDirections::Right => point = (point.0 + 1, point.1),
                }
            }
            break grid;
        };
        // TODO add random characters in remaining space

        GeneratedGame {
            grid: target_word_grid,
            valid_words: vec![String::from("todo")],
        }
    }
}
