//! Part 1

use std::str::FromStr;

use super::{common::Assignments, INPUT};

pub fn run() -> usize {
    Assignments::from_str(INPUT)
        .unwrap()
        .fully_contained_count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 582)
    }
}
