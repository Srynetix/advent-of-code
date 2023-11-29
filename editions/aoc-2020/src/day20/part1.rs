//! Part 1

use super::{
    common::{TileMatcher, TileParser},
    INPUT,
};

pub fn run() -> usize {
    let tiles = TileParser::parse_multiple_from_input(INPUT);
    TileMatcher::find_puzzle_corners(&tiles).iter().product()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 140_656_720_229_539);
    }
}
