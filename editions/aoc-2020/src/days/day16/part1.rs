//! Part 1

use super::{common::InputParser, INPUT};

pub fn run() -> usize {
    InputParser::from(INPUT)
        .validate_nearby_tickets()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 22_000);
    }
}
