//! Part 2

use super::{common::GameWithJoker, INPUT};

pub fn run() -> usize {
    GameWithJoker::from_input(INPUT).total_winnings()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 246_285_222)
    }
}
