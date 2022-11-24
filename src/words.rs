use rand::{seq::SliceRandom, thread_rng};
use std::fs;

pub fn read_words() -> Result<Vec<String>, std::io::Error> {
    let text = fs::read_to_string("./words.txt")?;
    let vec: Vec<String> = text.lines().map(|w| w.to_string()).collect();
    Ok(vec)
}

pub fn get_all_n_length_words(words: &Vec<String>, n: usize) -> Vec<String> {
    let mut output = Vec::new();

    for word in words.iter() {
        if word.chars().count() == n {
            output.push(word.to_string())
        }
    }
    output
}

pub fn get_random_word(words: &Vec<String>) -> Option<&String> {
    words.choose(&mut thread_rng())
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
}
