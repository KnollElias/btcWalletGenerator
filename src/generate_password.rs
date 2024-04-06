use rand::Rng;
use std::fs;

fn get_random_number(min: Option<i32>, max: Option<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    match (min, max) {
        (Some(min_val), Some(max_val)) => rng.gen_range(min_val..=max_val),
        _ => rng.gen_range(0..=2047),
    }
}

fn convert_number_to_word(number: i32) -> String {
    let words = fs::read_to_string("src/word_list/english.txt").expect("Unable to read file");
    let words: Vec<&str> = words.split('\n').collect();
    let word_position = number - 1;
    words[word_position as usize].to_string()
}

fn get_random_word() -> String {
    let min = Some(0);
    let max = Some(2047);

    let random_number = get_random_number(min, max);
    let random_word = convert_number_to_word(random_number);
    random_word.to_string()
}

pub fn get_random_phrase() -> Vec<String> {
    let length = 12;
    let mut result_phrase = Vec::new();
    for _ in 0..length {
        let word = get_random_word();
        result_phrase.push(word);
    }
    result_phrase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_number() {
        let min = 1;
        let max = 2047;
        let random_number = get_random_number(Some(min), Some(max));
        assert!(random_number >= min && random_number <= max, "Random number is within specified range.");
    }

    #[test]
    fn test_convert_number_to_word() {
        let expected_word = "abandon";
        let word = convert_number_to_word(1);
        assert_eq!(word, expected_word, "First word should be '{}'.", expected_word);
    }

    #[test]
    fn test_get_random_word() {
        let word = get_random_word();
        assert!(!word.is_empty(), "Random word should not be empty.");
    }

    #[test]
    fn test_get_random_phrase_length() {
        let phrase = get_random_phrase();
        assert_eq!(phrase.len(), 12, "Phrase should contain 12 words.");
    }
}