//! Part 1

use std::str::FromStr;

use super::{INPUT, common::RucksackParser};

pub fn run() -> u32 {
    RucksackParser::from_str(INPUT).unwrap().priority_sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 7_553)
    }
}
