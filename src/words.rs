use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};
use std::fs;

lazy_static! {
    static ref WORDS: Vec<String> = read_words();
}

/// Reads the result of `words.txt` into a `Result<Vec<String>>` of its contents
fn read_words() -> Vec<String> {
    let text = fs::read_to_string("./words.txt").expect(
        "Wordlist not found! Please place a `words.txt` wordlist file in the root directory.",
    );
    text.lines().map(|w| w.to_lowercase()).collect()
}

/// Provided a reference of `words`, and a size `n`, returns a vector of all words of
/// that size.
///
/// Words is passed as an argument to prevent having to reread the file, this could
/// be refactored later.
fn get_all_n_length_words(n: usize) -> Vec<String> {
    let mut output = Vec::new();

    for word in WORDS.iter() {
        if word.chars().count() == n {
            output.push(word.to_string())
        }
    }
    output
}

/// Given a vector of `words`, randomly selects one.
pub fn get_random_word(words: &Vec<String>) -> Option<&String> {
    words.choose(&mut thread_rng())
}

/// Generates a random letter
pub fn get_random_letter() -> char {
    ('a'..='z').choose(&mut thread_rng()).unwrap()
}

/// Generates a random word of length `n`, from the wordlist
pub fn get_random_n_length_word(n: usize) -> String {
    get_random_word(&get_all_n_length_words(n))
        .expect("Requested word of nonexistant size!")
        .clone()
}

/// Given `words` and `characters`, returns a new vector of only words solely comprised of
/// those characters
///
/// Hopefully, this means that when we need to search for permutations of words, this makes
/// it significantly cheaper
fn filter_words_by_character(characters: &[char]) -> Vec<String> {
    WORDS
        .iter()
        .filter_map(|w| {
            if w.chars().all(|c| characters.contains(&c)) {
                return Some(w.to_string());
            }
            None
        })
        .collect()
}

/// Given a game `&grid`, returns a vector of all the words that can be found inside that grid.
pub fn generate_wordlist_from_game<const N: usize>(grid: &[[char; N]; N]) -> Vec<String> {
    /// Recursive helper function to search for the remaining `word` slice in the `grid`.
    fn search_for_word<const N: usize>(
        grid: &[[char; N]; N],
        word: &str,
        (y, x): (usize, usize),
        visited_squares: &mut [[bool; N]; N],
    ) -> bool {
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
            && x < N - 2
            && grid[y - 1][x + 1] == next_char
            && !visited_squares[y - 1][x + 1]
            && search_for_word(grid, &word[1..], (y - 1, x + 1), visited_squares)
        {
            return true;
        }
        // down
        if y < N - 2
            && grid[y + 1][x] == next_char
            && !visited_squares[y + 1][x]
            && search_for_word(grid, &word[1..], (y + 1, x), visited_squares)
        {
            return true;
        }
        // down left
        if y < N - 2
            && x > 0
            && grid[y + 1][x - 1] == next_char
            && !visited_squares[y + 1][x - 1]
            && search_for_word(grid, &word[1..], (y + 1, x - 1), visited_squares)
        {
            return true;
        }
        // down right
        if y < N - 2
            && x < N - 2
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
        if x < N - 2
            && grid[y][x + 1] == next_char
            && !visited_squares[y][x + 1]
            && search_for_word(grid, &word[1..], (y, x + 1), visited_squares)
        {
            return true;
        }
        false
    }

    let mut letters = (*grid).into_iter().flatten().collect::<Vec<_>>();
    letters.sort();
    letters.dedup();

    let possible_words = filter_words_by_character(&letters);
    let mut wordlist: Vec<String> = Vec::new();

    'words: for word in possible_words {
        let first_char = word.chars().next().unwrap();

        for (y, row) in grid.iter().enumerate() {
            for (x, grid_character) in row.iter().enumerate() {
                if *grid_character == first_char {
                    let mut visited_squares = [[false; N]; N];
                    if search_for_word(grid, &word[1..], (y, x), &mut visited_squares) {
                        wordlist.push(word);
                        continue 'words;
                    }
                }
            }
        }
    }
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
