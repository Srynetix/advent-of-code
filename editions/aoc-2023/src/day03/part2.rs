//! Part 2

use super::{common::Schematic, INPUT};

pub fn run() -> u32 {
    Schematic::from_input(INPUT).sum_of_gear_ratios()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 91_031_374)
    }
}
