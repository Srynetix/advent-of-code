//! Part 2

use super::{INPUT, common::Universe};

pub fn run() -> usize {
    Universe::from_input(INPUT)
        .expand(1_000_000)
        .sum_shortest_paths()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 560_822_911_938)
    }
}
