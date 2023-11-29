//! Common

use std::{convert::Infallible, ops::RangeInclusive, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
pub struct AssignmentPair {
    range_a: RangeInclusive<usize>,
    range_b: RangeInclusive<usize>,
}

impl AssignmentPair {
    pub fn is_fully_contained(&self) -> bool {
        Self::_is_a_fully_contained_in_b(&self.range_a, &self.range_b)
            || Self::_is_a_fully_contained_in_b(&self.range_b, &self.range_a)
    }

    pub fn is_overlapping(&self) -> bool {
        Self::_is_a_overlapping_b(&self.range_a, &self.range_b)
            || Self::_is_a_overlapping_b(&self.range_b, &self.range_a)
    }

    fn _is_a_fully_contained_in_b(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
        a.start() >= b.start() && a.end() <= b.end()
    }

    fn _is_a_overlapping_b(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
        a.start().max(b.start()) <= a.end().min(b.end())
    }
}

impl FromStr for AssignmentPair {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(s: &str) -> RangeInclusive<usize> {
            let s = s.split('-').collect::<Vec<_>>();
            RangeInclusive::new(s[0].parse().unwrap(), s[1].parse().unwrap())
        }

        let s = s.split(',').collect::<Vec<_>>();
        Ok(Self {
            range_a: parse_range(s[0]),
            range_b: parse_range(s[1]),
        })
    }
}

pub struct Assignments {
    pairs: Vec<AssignmentPair>,
}

impl Assignments {
    pub fn fully_contained_count(&self) -> usize {
        self.pairs
            .iter()
            .filter(|assignment| assignment.is_fully_contained())
            .count()
    }

    pub fn overlapping_count(&self) -> usize {
        self.pairs
            .iter()
            .filter(|assignment| assignment.is_overlapping())
            .count()
    }
}

impl FromStr for Assignments {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pairs: s
                .lines()
                .map(|s| AssignmentPair::from_str(s).unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use aoc_sx::indoc::indoc;

    use super::Assignments;

    const SAMPLE: &str = indoc! {r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "#};

    #[test]
    fn test_sample() {
        let assignments = Assignments::from_str(SAMPLE).unwrap();
        assert_eq!(assignments.fully_contained_count(), 2);
        assert_eq!(assignments.overlapping_count(), 4);
    }
}
