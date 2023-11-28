//! Common

use aoc_sx::itertools::Itertools;

type PublicKey = usize;

pub fn parse_keys(input: &str) -> (PublicKey, PublicKey) {
    input
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn transform_subject_number_iteration(i: usize, subject: usize) -> usize {
    (i * subject) % 20_201_227
}

pub fn transform_subject_number_loop(subject: usize, loop_size: usize) -> usize {
    (0..loop_size)
        .into_iter()
        .fold(1, |acc, _| transform_subject_number_iteration(acc, subject))
}

pub fn determine_loop_size(public_key: PublicKey) -> usize {
    let mut num = 1;
    let mut counter = 0;

    loop {
        num = transform_subject_number_iteration(num, 7);
        counter += 1;

        if num == public_key {
            break;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc!(
        "
            5764801
            17807724
        "
    );

    #[test]
    fn test_parse_keys() {
        assert_eq!(parse_keys(SAMPLE), (5_764_801, 17_807_724));
    }

    #[test]
    fn test_determine_loop_size() {
        assert_eq!(determine_loop_size(5_764_801), 8);
        assert_eq!(determine_loop_size(17_807_724), 11);
    }

    #[test]
    fn test_transform_subject_number_loop() {
        assert_eq!(transform_subject_number_loop(17_807_724, 8), 14_897_079);
        assert_eq!(transform_subject_number_loop(5_764_801, 11), 14_897_079);
    }
}
