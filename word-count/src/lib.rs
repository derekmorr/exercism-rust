use std::collections::HashMap;
use std::str::FromStr;

fn process_word(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for word in String::from_str(s).unwrap().split_whitespace() {
        let key = process_word(word);
        if !key.is_empty() {
            *map.entry(key).or_insert(0) += 1;
        }
    }

    map
}
