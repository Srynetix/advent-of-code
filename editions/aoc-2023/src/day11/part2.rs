//! Part 2

use super::{common::Universe, INPUT};

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
