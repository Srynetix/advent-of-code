//! Part 1

use super::{common::validate_multiple_passwords_with_count, INPUT};

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
