fn container(i: i8) -> String {
    match i {
        1 => "bottle".to_string(),
        _ => "bottles".to_string()
    }
}

fn next_count(i: i8) -> i8 {
    match i {
        0 => 99,
        _ => i - 1
    }
}

fn next_string(i: i8) -> String {
    match next_count(i) {
        0 => "no more bottles".to_string(),
        _ => format!("{} {}", next_count(i), container(i-1))
    }
}
fn pronoun(i: i8) -> String {
    match i {
        1 => "it".to_string(),
        _ => "one".to_string()
    }
}

pub fn verse(i: i8) -> String {
    if i == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else {
        format!("{i} {cont} of beer on the wall, {i} {cont} of beer.\nTake {pro} down and pass it around, {thing} of beer on the wall.\n",
         i=i, cont=container(i), pro=pronoun(i), thing=next_string(i))
    }
}

pub fn sing(start: i8, stop: i8) -> String {
    let range = (stop..start+1).rev();
    let verses: Vec<String> = range.map(|i| verse(i)).collect();
    verses.join("\n")
}
