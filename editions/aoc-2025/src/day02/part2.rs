//! Part 2

use crate::day02::common::check_invalid_patterns_complex_many;

use super::INPUT;

pub fn run() -> usize {
    check_invalid_patterns_complex_many(&INPUT.trim().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::day02::common::check_invalid_patterns_complex_many;

    use super::super::common::ProductIdRanges;

    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_check_invalid_patterns_complex_many() {
        let ranges = SAMPLE.parse::<ProductIdRanges>().unwrap();
        assert_eq!(check_invalid_patterns_complex_many(&ranges), 4_174_379_265);
    }

    #[test]
    fn run() {
        assert_eq!(super::run(), 53_481_866_137);
    }
}
