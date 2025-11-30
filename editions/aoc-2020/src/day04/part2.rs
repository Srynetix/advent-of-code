//! Part 2

use super::{INPUT, common::PassportValidator};

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
