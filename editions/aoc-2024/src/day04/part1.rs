//! Part 1

use super::{common::Puzzle, INPUT};

pub fn run() -> usize {
    Puzzle::from_input(INPUT)
        .to_square_window()
        .count_word_in_window("XMAS") as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2507);
    }
}
