extern crate chrono;
use chrono::*;

pub fn after(d1: DateTime<UTC>) -> DateTime<UTC> {
    d1 + Duration::seconds(1_000_000_000)
}
