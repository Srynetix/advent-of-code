//! Part 1

use super::{INPUT, common::XmasScanner};

pub fn run() -> usize {
    XmasScanner::parse_and_find_error(INPUT, 25).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 556_543_474);
    }
}
