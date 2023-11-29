//! Part 2

use std::str::FromStr;

use super::{common::GameParser, INPUT};

pub fn run() -> u32 {
    GameParser::from_str(INPUT)
        .unwrap()
        .total_score_from_outcome()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 14_470)
    }
}
