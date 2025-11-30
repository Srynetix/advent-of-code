//! Part 1

use super::{INPUT, common::MemoryGame};

pub fn run() -> usize {
    MemoryGame::from_str_input(INPUT).run_steps(2020)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 206);
    }
}
