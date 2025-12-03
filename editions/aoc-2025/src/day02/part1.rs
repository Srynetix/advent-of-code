//! Part 1

use super::INPUT;
use super::common::check_invalid_patterns_many;

pub fn run() -> usize {
    check_invalid_patterns_many(&INPUT.trim().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::super::common::ProductIdRanges;
    use super::*;

    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_check_invalid_patterns_many() {
        let ranges = SAMPLE.parse::<ProductIdRanges>().unwrap();
        assert_eq!(check_invalid_patterns_many(&ranges), 1_227_775_554);
    }

    #[test]
    fn run() {
        assert_eq!(super::run(), 44_487_518_055);
    }
}
