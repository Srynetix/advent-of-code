//! Part 1

use super::{common::BoardingPass, INPUT};

pub fn run() -> usize {
    BoardingPass::from_entries(INPUT)
        .iter()
        .map(BoardingPass::get_seat_id)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 818);
    }
}
