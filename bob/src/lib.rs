pub fn reply(s: &str) -> String {
  let all_caps = s.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());

  if s.is_empty() {
      "Fine. Be that way!".to_string()
  } else if all_caps {
      "Whoa, chill out!".to_string()
  } else if s.ends_with("?") {
      "Sure.".to_string()
  } else {
      "Whatever."
  }
}
