//! Part 2

use super::{common::validate_multiple_passwords_with_position, INPUT};

pub fn run() -> usize {
    validate_multiple_passwords_with_position(INPUT)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 616);
    }
}
