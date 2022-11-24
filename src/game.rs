use rand::{seq::IteratorRandom, thread_rng};

use crate::words::get_random_n_length_word;

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
            (x, y): &(usize, usize),
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

        let mut grid = [['0'; N]; N];
        loop {
            let start_point = (
                (0..N).into_iter().choose(&mut thread_rng()),
                (0..N).into_iter().choose(&mut thread_rng()),
            );
        }
    }
}
