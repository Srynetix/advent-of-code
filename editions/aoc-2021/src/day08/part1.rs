//! Part 1

use aoc_sx::algo::parse::parse_lines;

use super::{
    common::{PatternCounter, PatternLine},
    INPUT,
};

pub fn run() -> usize {
    let lines: Vec<PatternLine> = parse_lines(INPUT);
    PatternCounter::count_unambiguous_output_patterns(&lines)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 412)
    }
}
