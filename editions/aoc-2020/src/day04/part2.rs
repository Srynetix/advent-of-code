//! Part 2

use super::{common::PassportValidator, INPUT};

pub fn run() -> usize {
    PassportValidator::parse_entries(INPUT)
        .iter()
        .filter(|x| x.is_valid_full())
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 147);
    }
}
