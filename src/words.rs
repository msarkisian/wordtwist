use rand::{seq::SliceRandom, thread_rng};
use std::fs;

/// Reads the result of `words.txt` into a `Result<Vec<String>>` of its contents
pub fn read_words() -> Result<Vec<String>, std::io::Error> {
    let text = fs::read_to_string("./words.txt")?;
    let vec: Vec<String> = text.lines().map(|w| w.to_string()).collect();
    Ok(vec)
}

/// Provided a reference of `words`, and a size `n`, returns a vector of all words of
/// that size.
///
/// Words is passed as an argument to prevent having to reread the file, this could
/// be refactored later.
pub fn get_all_n_length_words(words: &Vec<String>, n: usize) -> Vec<String> {
    let mut output = Vec::new();

    for word in words.iter() {
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

/// Given `words` and `characters`, returns a new vector of only words solely comprised of
/// those characters
///
/// Hopefully, this means that when we need to search for permutations of words, this makes
/// it significantly cheaper
fn filter_words_by_character(words: &Vec<String>, characters: &[char]) -> Vec<String> {
    words
        .clone()
        .into_iter()
        .filter(|w| w.chars().all(|c| characters.contains(&c)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_lengthed_words() {
        let words: Vec<String> = read_words().unwrap();

        let twelve_length_words = get_all_n_length_words(&words, 12);
        for _word in twelve_length_words.iter() {
            // println!("{}", word)
        }
        let fifteen_length_words = get_all_n_length_words(&words, 15);
        for _word in fifteen_length_words.iter() {
            // println!("{}", word)
        }
    }

    #[test]
    fn get_random_n_length_word() {
        let words: Vec<String> = read_words().unwrap();
        let twelve_length_words = get_all_n_length_words(&words, 12);
        let random_twelve_length_word = get_random_word(&twelve_length_words);

        assert_eq!(random_twelve_length_word.unwrap().chars().count(), 12);
    }

    #[test]
    fn test_filter_words_by_characters() {
        let words: Vec<String> = read_words().unwrap();
        let four_length_words = get_all_n_length_words(&words, 4);
        let chars = ['r', 'u', 's', 't'];

        let filtered_four_length_words = filter_words_by_character(&four_length_words, &chars);

        println!("{:?}", filtered_four_length_words);
        assert_eq!(filtered_four_length_words, vec!["rust", "ruts"]);
    }
}
