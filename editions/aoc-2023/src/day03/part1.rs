//! Part 1

use super::{INPUT, common::Schematic};

pub fn run() -> u32 {
    Schematic::from_input(INPUT).sum_of_part_numbers()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 546_563);
    }
}
