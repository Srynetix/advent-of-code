//! Part 1

use aoc_sx::algo::parse::parse_lines;

use super::{common::count_increments, INPUT};

pub fn run() -> usize {
    count_increments(&parse_lines(INPUT))
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1532)
    }
}
