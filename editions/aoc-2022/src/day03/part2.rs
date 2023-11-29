//! Part 2

use std::str::FromStr;

use super::{common::RucksackParser, INPUT};

pub fn run() -> u32 {
    RucksackParser::from_str(INPUT)
        .unwrap()
        .group_priority_sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2_758)
    }
}
