use std::{error::Error, fs};

fn read_words() -> Result<String, std::io::Error> {
    fs::read_to_string("./words.txt")
}

fn find_n_length_words(n: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let words = read_words()?;

    let mut output = Vec::new();

    for word in words.lines() {
        if word.chars().count() == n {
            output.push(word.to_string())
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_length_12_words() {
        let words = find_n_length_words(12).unwrap();
        for word in &words {
            println!("{}", word)
        }
    }
}
