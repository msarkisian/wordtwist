use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};
use std::{fs, thread};

const THREADS: usize = 8;

lazy_static! {
    static ref WORDS: Vec<String> = read_words();
}

/// Reads the result of `words.txt` into a vector of strings.
fn read_words() -> Vec<String> {
    let text = fs::read_to_string("./words.txt").expect(
        "Wordlist not found! Please place a `words.txt` wordlist file in the crate directory.",
    );
    text.lines().map(|w| w.to_lowercase()).collect()
}

/// Provided a size `n`, returns a vector of all words of that size, using the module's `lazy_static!` `WORDS`.
fn get_all_n_length_words(n: usize) -> Vec<String> {
    let mut output = Vec::new();

    for word in WORDS.iter() {
        if word.chars().count() == n {
            output.push(word.to_string())
        }
    }
    output
}

/// Randomly selects from `words`.
pub fn get_random_word(words: &[String]) -> Option<&String> {
    words.choose(&mut thread_rng())
}

/// Generates a vowel-weighted random letter.
///
/// Vowels are twice as likely to be selected as consonants.
pub fn get_random_letter() -> char {
    ('a'..='z')
        .chain(['a', 'e', 'i', 'o', 'u'])
        .choose(&mut thread_rng())
        .unwrap()
}

/// Generates a random word of length `n` from the module's `lazy_static!` `WORDS`.
pub fn get_random_n_length_word(n: usize) -> String {
    get_random_word(&get_all_n_length_words(n))
        .expect("Requested word of nonexistant size!")
        .clone()
}

/// Given `words` and `characters`, returns a new vector of only words solely comprised of
/// those characters.
///
/// This is to trim down the possible words to search for in grid permutations to a managable amount.
///
/// Future optimization here (e.g. with character number count) could increase performance further.
///
/// This returns a Vec of Options, so they can be removed later in a multithreaded context.
fn filter_words_by_character(characters: &[char]) -> Vec<Option<String>> {
    WORDS
        .iter()
        .filter_map(|w| {
            if w.chars().all(|c| characters.contains(&c)) {
                return Some(Some(w.to_string()));
            }
            None
        })
        .collect()
}

/// Given a game `&grid`, returns a vector of all the words that can be found inside that grid.
///
/// This function uses fork-join multithreading to increase performance, using the thread count set by the
/// `THREADS` const of the module.
pub fn generate_wordlist_from_game(grid: &Vec<Vec<char>>) -> Vec<String> {
    /// Recursive helper function to search for the remaining `word` slice in the `grid`.
    fn search_for_word(
        grid: &Vec<Vec<char>>,
        word: &str,
        (y, x): (usize, usize),
        visited_squares: &mut Vec<Vec<bool>>,
    ) -> bool {
        let grid_length = grid.len();
        visited_squares[y][x] = true;

        let next_char = match word.chars().next() {
            None => return true,
            Some(char) => char,
        };

        // up
        if y > 0
            && grid[y - 1][x] == next_char
            && !visited_squares[y - 1][x]
            && search_for_word(grid, &word[1..], (y - 1, x), visited_squares)
        {
            return true;
        }
        // up left
        if y > 0
            && x > 0
            && grid[y - 1][x - 1] == next_char
            && !visited_squares[y - 1][x - 1]
            && search_for_word(grid, &word[1..], (y - 1, x - 1), visited_squares)
        {
            return true;
        }
        // up right
        if y > 0
            && x < grid_length - 1
            && grid[y - 1][x + 1] == next_char
            && !visited_squares[y - 1][x + 1]
            && search_for_word(grid, &word[1..], (y - 1, x + 1), visited_squares)
        {
            return true;
        }
        // down
        if y < grid_length - 1
            && grid[y + 1][x] == next_char
            && !visited_squares[y + 1][x]
            && search_for_word(grid, &word[1..], (y + 1, x), visited_squares)
        {
            return true;
        }
        // down left
        if y < grid_length - 1
            && x > 0
            && grid[y + 1][x - 1] == next_char
            && !visited_squares[y + 1][x - 1]
            && search_for_word(grid, &word[1..], (y + 1, x - 1), visited_squares)
        {
            return true;
        }
        // down right
        if y < grid_length - 1
            && x < grid_length - 1
            && grid[y + 1][x + 1] == next_char
            && !visited_squares[y + 1][x + 1]
            && search_for_word(grid, &word[1..], (y + 1, x + 1), visited_squares)
        {
            return true;
        }
        // left
        if x > 0
            && grid[y][x - 1] == next_char
            && !visited_squares[y][x - 1]
            && search_for_word(grid, &word[1..], (y, x - 1), visited_squares)
        {
            return true;
        }
        // right
        if x < grid_length - 1
            && grid[y][x + 1] == next_char
            && !visited_squares[y][x + 1]
            && search_for_word(grid, &word[1..], (y, x + 1), visited_squares)
        {
            return true;
        }
        visited_squares[y][x] = false;
        false
    }

    let mut letters: Vec<char> = grid.iter().flatten().copied().collect();
    letters.sort();
    letters.dedup();

    let mut possible_words = filter_words_by_character(&letters);
    let possible_words_len = possible_words.len();
    let mut possible_word_chunks = possible_words.chunks_mut((possible_words_len / THREADS) + 1);
    let mut wordlist: Vec<String> = Vec::new();

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(THREADS);
        for _ in 0..THREADS {
            let thread_possible_words = possible_word_chunks.next().unwrap();
            handles.push(s.spawn(move || {
                let mut thread_wordlist = Vec::new();
                'words: for word_option in thread_possible_words {
                    let word = word_option.take().unwrap();
                    let first_char = word.chars().next().unwrap();

                    for (y, row) in grid.iter().enumerate() {
                        for (x, grid_character) in row.iter().enumerate() {
                            if *grid_character == first_char {
                                let mut visited_squares =
                                    vec![vec![false; grid.len()]; grid[0].len()];
                                if search_for_word(grid, &word[1..], (y, x), &mut visited_squares) {
                                    thread_wordlist.push(word);
                                    continue 'words;
                                }
                            }
                        }
                    }
                }
                thread_wordlist
            }))
        }

        for handle in handles {
            wordlist.extend(handle.join().unwrap());
        }
    });
    wordlist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_lengthed_words() {
        let twelve_length_words = get_all_n_length_words(12);
        for _word in twelve_length_words.iter() {
            // println!("{}", word)
        }
        let fifteen_length_words = get_all_n_length_words(15);
        for _word in fifteen_length_words.iter() {
            // println!("{}", word)
        }
    }

    #[test]
    fn get_random_n_length_word() {
        let twelve_length_words = get_all_n_length_words(12);
        let random_twelve_length_word = get_random_word(&twelve_length_words);

        assert_eq!(random_twelve_length_word.unwrap().chars().count(), 12);
    }

    #[test]
    fn test_random_letter() {
        get_random_letter();
    }
}
