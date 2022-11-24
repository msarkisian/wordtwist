use std::fs;

fn read_words() -> Result<Vec<String>, std::io::Error> {
    let text = fs::read_to_string("./words.txt")?;
    let vec: Vec<String> = text.lines().map(|w| w.to_string()).collect();
    Ok(vec)
}

fn find_n_length_words(words: &Vec<String>, n: usize) -> Vec<String> {
    let mut output = Vec::new();

    for word in words.iter() {
        if word.chars().count() == n {
            output.push(word.to_string())
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_lengthed_words() {
        let words: Vec<String> = read_words().unwrap();

        let twelve_length_words = find_n_length_words(&words, 12);
        for word in twelve_length_words.iter() {
            println!("{}", word)
        }
        let fifteen_length_words = find_n_length_words(&words, 15);
        for word in fifteen_length_words.iter() {
            println!("{}", word)
        }
    }
}
