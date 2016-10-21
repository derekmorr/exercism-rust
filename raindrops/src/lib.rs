pub fn raindrops(i: i32) -> String {

    let mut output = String::new();

    if i % 3 == 0 {
        output.push_str("Pling");
    }

    if i % 5 == 0 {
        output.push_str("Plang");
    }

    if i % 7 == 0 {
        output.push_str("Plong");
    }

    if output.is_empty() {
        output = i.to_string();
    }

    output
}
