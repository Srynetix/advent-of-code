//! Part 2

use super::{common::{TileParser, TileMatcher}, INPUT};

pub fn run() -> usize {
    let tiles = TileParser::parse_multiple_from_input(INPUT);
    let puzzle = TileMatcher::build_puzzle(&tiles);
    let puzzle = TileMatcher::find_and_replace_sea_monsters(&puzzle).unwrap();

    puzzle.0.iter().fold(0, |acc, x| {
        acc + x
            .iter()
            .fold(0, |acc, x| if *x == '#' { acc + 1 } else { acc })
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1885);
    }
}
