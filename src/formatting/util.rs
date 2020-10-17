use std::cmp::Ordering;

pub fn longest_hint<'a>(hint1: &'a str, hint2: &'a str) -> &'a str {
    return match hint1.len().cmp(&hint2.len()) {
        Ordering::Less => hint2,
        Ordering::Greater => hint1,
        Ordering::Equal => hint1,
    }
}

