use std::collections::HashMap;

/// Creates a dictionary from a string input.
fn create_alien_dict(input: &str) -> HashMap<char, usize> {
    input
        .chars()
        .enumerate()
        .map(|(index, char)| (char, index))
        .collect::<HashMap<char, usize>>()
}

/// Compare if these two words are sorted according to the alien dict result `create_alien_dict`
/// 1. Set the min length of the two words
/// 2. Compare each word, we need to have an ascendant order.
/// 3. If we have similar words, we compare the length of the words.
fn verify_order(first_word: &str, second_word: &str, dict: &HashMap<char, usize>) -> bool {
    let min_length = first_word.len().min(second_word.len());
    for i in 0..min_length {
        if let (Some(first_word_char), Some(second_word_char)) =
            (first_word.chars().nth(i), second_word.chars().nth(i))
        {
            match (dict.get(&first_word_char), dict.get(&second_word_char)) {
                (Some(&pos1), Some(&pos2)) => {
                    if pos1 == pos2 {
                        continue;
                    }
                    return pos1 < pos2;
                }
                _ => {}
            }
        }
    }

    return first_word.len() <= second_word.len();
}

fn is_alien_sorted(alphabet: &str, words: &[&str]) -> bool {
    let dict = create_alien_dict(alphabet);
    for i in 1..words.len() {
        if !verify_order(words[i - 1], words[i], &dict) {
            return false;
        }
    }
    return true;
}

fn main() {
    let is_sorted = is_alien_sorted("abcedfghijklmnopqrstuvwxyz", &["d", "dd", "ddd", "dddf"]);

    println!("{}", is_sorted)
}
