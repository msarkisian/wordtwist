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
        let target_word_size = (2 * N..3 * N)
            .into_iter()
            .choose(&mut thread_rng())
            .unwrap();
        let target_word = get_random_n_length_word(target_word_size);

        let mut grid = [['0'; N]; N];
        let start_point = (
            (0..N).into_iter().choose(&mut thread_rng()),
            (0..N).into_iter().choose(&mut thread_rng()),
        );
    }
}
