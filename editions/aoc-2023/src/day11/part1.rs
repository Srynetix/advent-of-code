//! Part 1

use super::{INPUT, common::Universe};

pub fn run() -> usize {
    Universe::from_input(INPUT).expand(1).sum_shortest_paths()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 10_033_566)
    }
}
