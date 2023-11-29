//! Part 2

use super::{common::TobogganMap, INPUT};

pub fn run() -> usize {
    let map = TobogganMap::from_input(INPUT);

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(offset_x, offset_y)| map.follow_slope(*offset_x, *offset_y))
        .product()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 3_621_285_278);
    }
}
