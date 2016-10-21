pub fn hamming_distance(s1: &str, s2: &str) -> Result<u32, String> {
    if s1.len() != s2.len() {
        Err("sequences are of unequal lenth".to_string())
    } else {
        let pairs = s1.chars().zip(s2.chars());
        let hamming_count = pairs.fold(0, |acc, p| if p.0 == p.1 { acc } else { acc + 1} );
        Ok(hamming_count)
    }
}
