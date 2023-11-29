//! Common

use std::cmp::Ordering;

use aoc_sx::itertools::Itertools;

/// Xmas Weakness output
pub struct XmasWeaknessOutput {
    data: Vec<usize>,
    start_cursor: usize,
    end_cursor: usize,
}

impl XmasWeaknessOutput {
    /// Create output from data, start and end cursor.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    /// * `start_cursor` - Start cursor
    /// * `end_cursor` - End cursor
    pub fn new(data: Vec<usize>, start_cursor: usize, end_cursor: usize) -> Self {
        Self {
            data,
            start_cursor,
            end_cursor,
        }
    }

    /// Find smallest and largest value in data.
    pub fn get_min_max(&self) -> (usize, usize) {
        let range = &self.data[self.start_cursor..self.end_cursor];
        (*range.iter().min().unwrap(), *range.iter().max().unwrap())
    }

    /// Get sum of min and max value.
    pub fn get_sum(&self) -> usize {
        let min_max = self.get_min_max();
        min_max.0 + min_max.1
    }
}

/// Xmas scanner
pub struct XmasScanner;

impl XmasScanner {
    /// Parse input string and find error if any.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_and_find_error(input: &str, ring_size: usize) -> Option<usize> {
        let mut data = Vec::with_capacity(ring_size);
        // Parse initial data
        let mut initial_data = input
            .lines()
            .take(ring_size)
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect();
        data.append(&mut initial_data);

        // Now parse remaining items
        let remaining_items: Vec<usize> = input
            .lines()
            .skip(ring_size)
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect();

        for x in remaining_items {
            if Self::find_sum(&data, x).is_some() {
                data.remove(0);
                data.push(x);
            } else {
                return Some(x);
            }
        }

        None
    }

    /// Find a sum of number from the ring equals to the target number.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    /// * `target` - Target number
    pub fn find_sum(data: &[usize], target: usize) -> Option<(usize, usize)> {
        data.iter().combinations(2).find_map(|v| {
            if v[0] + v[1] == target {
                Some((*v[0], *v[1]))
            } else {
                None
            }
        })
    }

    /// Find weakness.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    /// * `invalid_number` - Invalid number
    pub fn find_weakness(input: &str, invalid_number: usize) -> Option<XmasWeaknessOutput> {
        let numbers: Vec<usize> = input
            .lines()
            .filter_map(|x| x.trim().parse().ok())
            .collect();

        for start_i in 0..numbers.len() {
            let mut acc = numbers[start_i];

            for end_i in start_i + 1..numbers.len() {
                acc += numbers[end_i];

                match acc.cmp(&invalid_number) {
                    Ordering::Equal => {
                        return Some(XmasWeaknessOutput::new(numbers, start_i, end_i))
                    }
                    Ordering::Greater => {
                        break;
                    }
                    Ordering::Less => (),
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576"#;

    #[test]
    fn test_parse_and_find_error() {
        assert_eq!(XmasScanner::parse_and_find_error(SAMPLE, 5), Some(127));
    }

    #[test]
    fn test_find_weakness() {
        let weakness = XmasScanner::find_weakness(SAMPLE, 127).unwrap();
        assert_eq!(weakness.start_cursor, 2);
        assert_eq!(weakness.end_cursor, 5);
        assert_eq!(weakness.get_min_max(), (15, 47));
        assert_eq!(weakness.get_sum(), 62);
    }
}
