//! Common

use std::collections::HashMap;

/// Count unique questions where anyone answered 'yes' for group entries.
///
/// # Arguments
///
/// * `group_entries` - Group entries
pub fn count_unique_questions_for_anyone(group_entries: &str) -> usize {
    let mut entries: Vec<char> = group_entries
        .lines()
        .flat_map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    entries.sort_unstable();
    entries.dedup();
    entries.len()
}

/// Count unique questions where everyone answered 'yes' for group entries.
///
/// # Arguments
///
/// * `group_entries` - Group entries
pub fn count_unique_questions_for_everyone(group_entries: &str) -> usize {
    let lines: Vec<&str> = group_entries.lines().collect();
    let group_count = lines.len();

    let mut counter = HashMap::new();
    for l in lines {
        for c in l.chars() {
            let v = counter.entry(c).or_insert(0);
            *v += 1;
        }
    }

    counter.keys().filter(|k| counter[k] == group_count).count()
}
