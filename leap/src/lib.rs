pub fn is_leap_year(year: i32) -> bool {
    let divisble_by_4   = year % 4   == 0;
    let divisble_by_100 = year % 100 == 0;
    let divisble_by_400 = year % 400 == 0;

    (divisble_by_4 && !divisble_by_100) ||
        (divisble_by_4 && divisble_by_100 && divisble_by_400)
}
