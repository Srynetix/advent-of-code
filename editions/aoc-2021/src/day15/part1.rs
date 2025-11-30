//! Part 1

use super::{INPUT, common::Cave};

pub fn run() -> u64 {
    let cave = Cave::from(INPUT);
    cave.get_lower_risk_path_sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 621)
    }
}
