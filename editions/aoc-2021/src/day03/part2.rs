//! Part 2

use aoc_sx::algo::parse::parse_str_lines;

use super::{common::BitAnalyzer, INPUT};

pub fn run() -> u32 {
    let input = parse_str_lines(INPUT);
    let oxygen = BitAnalyzer::compute_oxygen_generator_rating(&input);
    let scrubber = BitAnalyzer::compute_co2_scrubber_rating(&input);
    oxygen * scrubber
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 7_440_311)
    }
}
