//! Part 1

use super::{
    INPUT,
    common::{BingoParser, BingoPlayer},
};

pub fn run() -> u32 {
    let (play, grids) = BingoParser::parse_play_and_grids(INPUT);
    BingoPlayer::play(play, grids)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 69_579)
    }
}
