//! Part 1

use super::{common::Almanac, INPUT};

pub fn run() -> usize {
    Almanac::from_input(INPUT).get_lowest_location_numbers_from_seeds()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1_181_555_926)
    }
}
