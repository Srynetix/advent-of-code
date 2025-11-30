//! Part 2

use super::{INPUT, common::ScratchcardPile};

pub fn run() -> usize {
    ScratchcardPile::from_input(INPUT).compute_cards_count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 10_212_704);
    }
}
