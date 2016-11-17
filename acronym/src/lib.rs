pub fn abbreviate(s: &str) -> String {
    fn process_word(w: &str) -> Vec<char> {
        w.chars().nth(0).unwrap().to_uppercase().nth(0).unwrap()
    }

    let words = s.split_whitespace();
    words.fold("".to_string(), |mut acc, e| { acc.push(process_word(e)); acc } )
}
