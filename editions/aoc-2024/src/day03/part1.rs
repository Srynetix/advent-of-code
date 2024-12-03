//! Part 1

use super::{common::Puzzle, INPUT};

pub fn run() -> usize {
    Puzzle::from_input(INPUT).compute_multiplications() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 178_886_550)
    }
}
