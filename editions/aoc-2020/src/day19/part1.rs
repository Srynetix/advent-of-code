//! Part 1

use super::{INPUT, common::RuleSystem};

pub fn run() -> usize {
    RuleSystem::from_rules_and_values(INPUT).len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 118);
    }
}
