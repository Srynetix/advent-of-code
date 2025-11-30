//! Part 1

use super::{INPUT, common::Game};

pub fn run() -> usize {
    Game::from_input(INPUT).total_winnings()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 248_113_761)
    }
}
