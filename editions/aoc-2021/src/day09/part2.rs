//! Part 2

use super::{common::HeightMap, INPUT};

pub fn run() -> usize {
    let hm = HeightMap::from(INPUT);
    let basins = hm.find_basin_positions();
    hm.largest_basin_mul(basins)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 821_560)
    }
}
