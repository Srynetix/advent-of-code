//! Part 2

use super::{common::JoltAnalyzer, INPUT};

pub fn run() -> usize {
    JoltAnalyzer::from_input(INPUT).count_adapter_permutations()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 3_454_189_699_072);
    }
}
