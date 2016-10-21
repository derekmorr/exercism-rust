use std::ascii::AsciiExt;

fn score_letter(c: char) -> u32 {
    match c.to_ascii_uppercase() {
        'A' | 'E' | 'I' | 'O' | 'U' => 1,
        'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G'                   => 2,
        'B' | 'C' | 'M' | 'P'       => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K'                         => 5,
        'J' | 'X'                   => 8,
        'Q' | 'Z'                   => 10,
        _                           => 0
    }
}

pub fn score(s: &str) -> u32 {
    s.chars().fold(0, |acc, e| acc + score_letter(e))
}
