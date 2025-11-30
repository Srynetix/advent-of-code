//! Part 1

use super::{INPUT, common::ScratchcardPile};

pub fn run() -> usize {
    ScratchcardPile::from_input(INPUT).get_total_points_value()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 28_750)
    }
}
