//! Part 2

use super::{
    INPUT,
    common::{Analyzer, Puzzle},
};

pub fn run() -> usize {
    Analyzer::count_safe_reports_with_dampener(&Puzzle::from_input(INPUT)) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 658)
    }
}
