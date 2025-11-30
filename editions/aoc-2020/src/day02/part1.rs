//! Part 1

use super::{INPUT, common::validate_multiple_passwords_with_count};

pub fn run() -> usize {
    validate_multiple_passwords_with_count(INPUT)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 418);
    }
}
