//! Part 1

use super::{common::count_unique_questions_for_everyone, INPUT};

pub fn run() -> usize {
    INPUT
        .split("\n\n")
        .map(count_unique_questions_for_everyone)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 3435);
    }
}
