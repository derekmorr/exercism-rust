pub fn square_of_sum(i: i32) -> i32 {
    let s: i32 = (1..i+1).sum();
    s * s
}

pub fn sum_of_squares(i: i32) -> i32 {
    (1..i+1).map(|x| x * x).sum()
}

pub fn difference(i: i32) -> i32 {
    square_of_sum(i) - sum_of_squares(i)
}
