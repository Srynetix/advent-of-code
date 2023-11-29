//! Part 1

use aoc_sx::algo::parse::parse_str_lines;

use super::{common::Submarine, INPUT};

pub fn run() -> i32 {
    Submarine::from_moves(&parse_str_lines(INPUT)).score()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2_102_357)
    }
}
