//! Part 1

use super::{common::HeightMap, INPUT};

pub fn run() -> u32 {
    let hm = HeightMap::from(INPUT);
    let points = hm.find_low_points();
    hm.compute_risk_level_sum(&points)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 631)
    }
}
