//! Part 2

use crate::day01::common::{Safe, parse_document};

use super::INPUT;

pub fn run() -> usize {
    let rotations = parse_document(INPUT);
    let mut safe = Safe::new();
    let mut zeroes = 0;

    for rotation in rotations {
        if rotation.degrees == 0 {
            continue;
        }

        zeroes += safe.rotate(rotation);
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"
    };

    #[test]
    fn test_sample_with_overlaps() {
        let rotations = parse_document(SAMPLE);
        let mut safe = Safe::new();
        let mut zeroes = 0;

        for rotation in rotations {
            if rotation.degrees == 0 {
                continue;
            }

            zeroes += safe.rotate(rotation);
        }

        assert_eq!(safe.peek(), 32);
        assert_eq!(zeroes, 6);
    }

    #[test]
    fn run() {
        assert_eq!(super::run(), 5963);
    }
}
