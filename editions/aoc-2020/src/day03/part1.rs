//! Part 1

use super::{common::TobogganMap, INPUT};

pub fn run() -> usize {
    let (x, y) = (3, 1);
    TobogganMap::from_input(INPUT).follow_slope(x, y)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 299);
    }
}
