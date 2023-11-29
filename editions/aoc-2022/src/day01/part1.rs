//! Part 1

use std::str::FromStr;

use super::{common::ElfCalorieReader, INPUT};

pub fn run() -> u32 {
    ElfCalorieReader::from_str(INPUT).unwrap().max_calories()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 66_616)
    }
}
