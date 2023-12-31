//! Part 2

use super::{common::Almanac, INPUT};

pub fn run() -> usize {
    Almanac::from_input(INPUT).get_lowest_location_numbers_using_reverse_bruteforce()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 37_806_486)
    }
}
