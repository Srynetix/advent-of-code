//! Part 2

use aoc_sx::algo::parse::parse_lines;

use super::{INPUT, common::count_increments_three};

pub fn run() -> usize {
    count_increments_three(&parse_lines(INPUT))
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1571)
    }
}
