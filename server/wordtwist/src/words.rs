use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

lazy_static! {
    static ref WORDS: Vec<String> = read_words();
}

/// Parses the compiled wordlist (from `words.txt`) into a vector of individual words.
fn read_words() -> Vec<String> {
    let text = include_str!("../words.txt");
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

/// Counts the number of each character of `word`, returning an array of counts
/// (where the 0th index is 'a')
fn count_chars(word: &str) -> [usize; 26] {
    let mut counts = [0; 26];
    for char in word.chars() {
        counts[(char as u8 - b'a') as usize] += 1;
    }
    counts
}

/// Given `words` and `characters`, returns a new vector of only words solely comprised of
/// those characters in the provided count.
///
/// This is to trim down the possible words to search for in grid permutations to a managable amount.
fn filter_words_by_character(characters: &str) -> Vec<String> {
    let char_count = count_chars(characters);
    WORDS
        .iter()
        .filter_map(|w| {
            let word_count = count_chars(w);
            if word_count
                .iter()
                .enumerate()
                .all(|(idx, count)| char_count[idx] >= *count)
            {
                return Some(w.to_string());
            }
            None
        })
        .collect()
}

/// Given a game `&grid`, returns a vector of all the words that can be found inside that grid.
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

    let letters: String = grid.iter().flatten().copied().collect();

    let possible_words = filter_words_by_character(&letters);
    let mut wordlist: Vec<String> = Vec::new();

    'words: for word in possible_words {
        let first_char = word.chars().next().unwrap();

        for (y, row) in grid.iter().enumerate() {
            for (x, grid_character) in row.iter().enumerate() {
                if *grid_character == first_char {
                    let mut visited_squares = vec![vec![false; grid.len()]; grid[0].len()];
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

    #[test]
    fn test_count_chars() {
        let input = "boot";
        let output = count_chars(input);
        assert_eq!(output[1], 1); // b
        assert_eq!(output[14], 2); // o
        assert_eq!(output[0], 0);
    }

    #[test]
    fn test_filter_words_by_character() {
        let words = filter_words_by_character("bos");
        println!("{:?}", words);
        assert!(words.contains(&"sob".to_string()));
        assert!(!words.contains(&"boss".to_string()));
    }
}
