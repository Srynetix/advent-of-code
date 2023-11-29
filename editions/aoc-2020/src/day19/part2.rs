//! Part 2

use super::{common::RuleSystem, INPUT};

pub fn run() -> usize {
    RuleSystem::from_rules_and_values_alternative(INPUT).len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 246);
    }
}
