//! Part 1

use super::{common::JoltAnalyzer, INPUT};

pub fn run() -> usize {
    let (diff1, diff3) = JoltAnalyzer::get_1x3_jolt_differences(
        &JoltAnalyzer::from_input(INPUT).determine_jolt_chain(),
    );

    diff1 * diff3
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1820);
    }
}
