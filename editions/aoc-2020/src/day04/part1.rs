//! Part 1

use super::{INPUT, common::PassportValidator};

pub fn run() -> usize {
    PassportValidator::parse_entries(INPUT)
        .iter()
        .filter(|x| x.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 213);
    }
}
