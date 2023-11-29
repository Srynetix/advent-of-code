//! Part 1

use aoc_sx::algo::parse::parse_str_lines;

use super::{common::BitAnalyzer, INPUT};

pub fn run() -> u32 {
    let (gamma, epsilon) = BitAnalyzer::compute_gamma_and_epsilon(&parse_str_lines(INPUT));
    gamma * epsilon
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 3_959_450)
    }
}
