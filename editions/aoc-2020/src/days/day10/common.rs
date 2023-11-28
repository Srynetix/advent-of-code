//! Common

use std::collections::{VecDeque, HashSet, HashMap};

use aoc_sx::itertools::Itertools;

/// Jolt analyzer
pub struct JoltAnalyzer {
    data: Vec<usize>,
}

impl JoltAnalyzer {
    /// Creates analyzer from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_input(input: &str) -> Self {
        let mut data: Vec<usize> = input
            .lines()
            .filter_map(|x| x.trim().parse().ok())
            .sorted()
            .collect();

        // Add charging outlet: 0
        data.insert(0, 0);
        // Add device builtin adapter
        data.push(Self::get_builtin_adapter_jolts(&data));

        Self { data }
    }

    /// Get device builtin adapter jolts.
    pub fn get_builtin_adapter_jolts(data: &[usize]) -> usize {
        data.iter().max().unwrap() + 3
    }

    /// Determine jolt chain.
    pub fn determine_jolt_chain(&self) -> Vec<(usize, usize)> {
        let mut remaining: VecDeque<usize> = self.data.clone().into();
        let mut available = self.data.clone();
        let mut chain = vec![];

        while let Some(num) = remaining.pop_front() {
            if let Some((diff, adapter, idx)) =
                Self::find_smallest_compatible_adapter(&available, num)
            {
                chain.push((diff, adapter));
                available.remove(idx);
            }
        }

        chain
    }

    /// Count adapter permutations.
    pub fn count_adapter_permutations(&self) -> usize {
        let max_value = self.data.last().unwrap();
        let data_set: HashSet<usize> = self.data.iter().copied().collect();
        let mut memory: HashMap<usize, usize> = HashMap::new();
        Self::count_inner_adapter_permutations(&data_set, 0, *max_value, &mut memory)
    }

    /// Recursively count inner adapter permutations.
    ///
    /// # Arguments
    ///
    /// * `data` - Adapter data set
    /// * `number` - Base number to search
    /// * `max_value` - Maximum value to found
    /// * `memory` - Memory to write permutations for better performance
    pub fn count_inner_adapter_permutations(
        data: &HashSet<usize>,
        number: usize,
        max_value: usize,
        memory: &mut HashMap<usize, usize>,
    ) -> usize {
        memory.get(&number).cloned().map_or_else(
            || {
                let memory_value = {
                    if number == max_value {
                        1
                    } else {
                        (number + 1..=number + 3)
                            .filter_map(|i| {
                                data.get(&i).map(|i| {
                                    Self::count_inner_adapter_permutations(
                                        data, *i, max_value, memory,
                                    )
                                })
                            })
                            .sum()
                    }
                };

                memory.insert(number, memory_value);
                memory_value
            },
            |v| v,
        )
    }

    /// Get 1-jolt differences and 3-jolt differences from chain.
    ///
    /// # Arguments
    ///
    /// * `chain` - Adapter chain
    pub fn get_1x3_jolt_differences(chain: &[(usize, usize)]) -> (usize, usize) {
        let mut diff1 = 0;
        let mut diff3 = 0;

        for c in chain.iter().map(|c| c.0) {
            match c {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => (),
            }
        }

        (diff1, diff3)
    }

    /// Find smallest compatible adapter.
    ///
    /// # Arguments
    ///
    /// * `remaining` - Remaining adapters
    /// * `target` - Jolt target
    pub fn find_smallest_compatible_adapter(
        remaining: &[usize],
        target: usize,
    ) -> Option<(usize, usize, usize)> {
        let mut compatible: Vec<(usize, usize, usize)> = remaining
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| {
                let diff = x.wrapping_sub(target);
                if diff > 0 && diff <= 3 {
                    Some((diff, *x, idx))
                } else {
                    None
                }
            })
            .collect();

        compatible.sort_unstable();
        compatible.first().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = r#"16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4"#;

    const SAMPLE2: &str = r#"28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3"#;

    #[test]
    fn test_builtin_adapter_jolts() {
        let data: Vec<usize> = SAMPLE1
            .lines()
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        assert_eq!(JoltAnalyzer::get_builtin_adapter_jolts(&data), 22)
    }

    #[test]
    fn test_determine_jolt_chain() {
        assert_eq!(
            JoltAnalyzer::from_input(SAMPLE1).determine_jolt_chain(),
            vec![
                (1, 1),
                (3, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (3, 10),
                (1, 11),
                (1, 12),
                (3, 15),
                (1, 16),
                (3, 19),
                (3, 22)
            ]
        )
    }

    #[test]
    fn test_get_1x3_jolt_differences() {
        let chain = JoltAnalyzer::from_input(SAMPLE1).determine_jolt_chain();
        assert_eq!(JoltAnalyzer::get_1x3_jolt_differences(&chain), (7, 5));
    }

    #[test]
    fn test_get_1x3_jolt_differences_larger() {
        let chain = JoltAnalyzer::from_input(SAMPLE2).determine_jolt_chain();
        assert_eq!(JoltAnalyzer::get_1x3_jolt_differences(&chain), (22, 10));
    }

    #[test]
    fn test_count_adapter_permutations() {
        assert_eq!(
            JoltAnalyzer::from_input(SAMPLE1).count_adapter_permutations(),
            8
        );
    }

    #[test]
    fn test_count_adapter_permutations_larger() {
        assert_eq!(
            JoltAnalyzer::from_input(SAMPLE2).count_adapter_permutations(),
            19208
        );
    }
}
