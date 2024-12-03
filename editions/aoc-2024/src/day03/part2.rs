//! Part 2

use super::{common::Puzzle, INPUT};

pub fn run() -> usize {
    Puzzle::from_input_with_conditionals(INPUT).compute_multiplications() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 87_163_705);
    }
}
