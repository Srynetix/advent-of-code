//! Part 1

use crate::day01::common::{Safe, parse_document};

use super::INPUT;

pub fn run() -> usize {
    let rotations = parse_document(INPUT);
    let mut safe = Safe::new();
    let mut zeroes = 0;

    for rotation in rotations {
        safe.rotate(rotation);
        if safe.peek() == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use crate::day01::common::*;

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
    fn rotate_safe() {
        let mut safe = Safe::new();
        assert_eq!(safe.peek(), 50);

        safe.rotate("L90".parse().unwrap());
        assert_eq!(safe.peek(), 60);
    }

    #[test]
    fn test_sample() {
        let rotations = parse_document(SAMPLE);
        let mut safe = Safe::new();
        let mut zeroes = 0;

        for rotation in rotations {
            safe.rotate(rotation);
            if safe.peek() == 0 {
                zeroes += 1;
            }
        }

        assert_eq!(safe.peek(), 32);
        assert_eq!(zeroes, 3);
    }

    #[test]
    fn run() {
        assert_eq!(super::run(), 1043);
    }
}
