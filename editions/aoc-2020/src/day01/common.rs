//! Common

use aoc_sx::itertools::Itertools;

/// Search first combination of length `combinations` which sum equals to `target`.
///
/// # Arguments
///
/// * `entries_content` - Input text
/// * `combinations` - Combinations length
/// * `target` - Target value
pub fn search_if_eq(entries_content: &str, combinations: usize, target: usize) -> Vec<usize> {
    entries_content
        .lines()
        .filter_map(|s| s.parse::<usize>().ok())
        .combinations(combinations)
        .find(|v| v.iter().sum::<usize>() == target)
        .unwrap_or_else(Vec::new)
}

#[cfg(test)]
mod tests {
    use super::search_if_eq;

    #[test]
    fn test_search_if_eq() {
        assert_eq!(search_if_eq("1234\n5678\n2020\n0", 2, 2020), vec![2020, 0]);
        assert_eq!(search_if_eq("1234\n5678", 2, 2020), vec![]);
        assert_eq!(search_if_eq("", 2, 2020), vec![]);
        assert_eq!(search_if_eq("1234", 2, 2020), vec![]);
    }
}
