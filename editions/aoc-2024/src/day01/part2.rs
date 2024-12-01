//! Part 2

use super::{common::Sorter, INPUT};

pub fn run() -> usize {
    let (v1, v2) = Sorter::from_input(INPUT);
    Sorter::total_similarity_score(&v1, &v2) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 21_142_653);
    }
}
