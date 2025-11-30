//! Part 2

use super::{INPUT, common::MemoryGame};

pub fn run() -> usize {
    MemoryGame::from_str_input(INPUT).run_steps(30_000_000)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 955);
    }
}
