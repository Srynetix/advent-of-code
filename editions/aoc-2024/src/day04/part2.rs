//! Part 2

use super::{INPUT, common::Puzzle};

pub fn run() -> usize {
    Puzzle::from_input(INPUT).count_x_mas() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1969)
    }
}
