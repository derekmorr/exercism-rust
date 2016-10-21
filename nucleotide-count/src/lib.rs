use std::ascii::AsciiExt;
use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> u64 {
    dna.chars().fold(0, |acc, e| if e == nucleotide { acc + 1} else { acc } )
}

fn update_count(counts: (usize, usize, usize, usize), ch: char) -> (usize, usize, usize, usize) {
    let (a, c, g, t) = counts;
    match ch.to_ascii_lowercase() {
        'a' => (a+1, c, g, t),
        'c' => (a, c+1, g, t),
        'g' => (a, c, g+1, t),
        't' => (a, c, g, t+1),
        _   => (a, c, g, t)
    }
}


pub fn nucleotide_counts(dna: &str) -> HashMap<char, usize> {

    let initial = (0, 0, 0, 0);

    let counts = dna.chars().fold(initial, |acc, e| update_count(acc, e));

    let mut map = HashMap::new();
    map.insert('A', counts.0);
    map.insert('C', counts.1);
    map.insert('G', counts.2);
    map.insert('T', counts.3);

    map
}
