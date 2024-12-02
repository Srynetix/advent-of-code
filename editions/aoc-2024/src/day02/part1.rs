//! Part 1

use super::{
    common::{Analyzer, Puzzle},
    INPUT,
};

pub fn run() -> usize {
    Analyzer::count_safe_reports(&Puzzle::from_input(INPUT)) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 624);
    }
}
