//! Part 1

use super::{common::BoatRaces, INPUT};

pub fn run() -> usize {
    BoatRaces::from_input(INPUT).product_ways_to_beat_records()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 0)
    }
}
