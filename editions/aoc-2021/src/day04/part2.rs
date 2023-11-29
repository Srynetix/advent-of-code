//! Part 2

use super::{
    common::{BingoParser, BingoPlayer},
    INPUT,
};

pub fn run() -> u32 {
    let (play, grids) = BingoParser::parse_play_and_grids(INPUT);
    BingoPlayer::play_waiting_for_last(play, grids)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 14_877)
    }
}
