//! Common

use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductIdRange {
    pub start: usize,
    pub end: usize,
}

pub struct ProductIdRanges(pub Vec<ProductIdRange>);

impl FromStr for ProductIdRanges {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(',')
            .map(|part| part.parse::<ProductIdRange>().unwrap())
            .collect();
        Ok(ProductIdRanges(ranges))
    }
}

impl FromStr for ProductIdRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            panic!("Invalid range format");
        }
        let start = parts[0].parse::<usize>().unwrap();
        let end = parts[1].parse::<usize>().unwrap();
        Ok(ProductIdRange { start, end })
    }
}

pub fn check_invalid_patterns(range: ProductIdRange) -> Vec<usize> {
    let mut patterns = Vec::new();

    for product_id in range.start..=range.end {
        let id_str = product_id.to_string();
        let (before, after) = id_str.split_at(id_str.len() / 2);

        if before == after {
            patterns.push(product_id);
        }
    }

    patterns
}

pub fn check_invalid_patterns_complex(range: ProductIdRange) -> Vec<usize> {
    let mut patterns = Vec::new();

    'products: for product_id in range.start..=range.end {
        let id_str = product_id.to_string();
        let len = id_str.len();

        'search: for window_size in 1..=(len / 2) {
            let chunks = id_str
                .chars()
                .chunks(window_size)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .collect::<Vec<String>>();
            for i in 1..chunks.len() {
                if chunks[i - 1] != chunks[i] {
                    continue 'search;
                }
            }

            patterns.push(product_id);
            continue 'products;
        }
    }

    patterns
}

pub fn check_invalid_patterns_many(ranges: &ProductIdRanges) -> usize {
    ranges
        .0
        .iter()
        .map(|range| check_invalid_patterns(*range).into_iter().sum::<usize>())
        .sum()
}

pub fn check_invalid_patterns_complex_many(ranges: &ProductIdRanges) -> usize {
    ranges
        .0
        .iter()
        .map(|range| {
            check_invalid_patterns_complex(*range)
                .into_iter()
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_product_id_range() {
        let range_str = "100-200";
        let range = range_str.parse::<ProductIdRange>().unwrap();
        assert_eq!(range.start, 100);
        assert_eq!(range.end, 200);
    }

    #[test]
    fn test_check_invalid_patterns() {
        assert_eq!(check_invalid_patterns("11-22".parse().unwrap()), &[11, 22]);

        assert_eq!(check_invalid_patterns("95-115".parse().unwrap()), &[99]);

        assert_eq!(check_invalid_patterns("998-1012".parse().unwrap()), &[1010]);
    }

    #[test]
    fn test_check_invalid_patterns_complex() {
        assert_eq!(
            check_invalid_patterns_complex("998-1012".parse().unwrap()),
            &[999, 1010]
        );
    }
}
