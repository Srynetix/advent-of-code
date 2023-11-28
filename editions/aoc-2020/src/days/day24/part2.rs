//! Part 2

use super::{common::{HexGrid, parse_paths}, INPUT};

pub fn run() -> usize {
    let mut grid = HexGrid::default();
    grid.follow_paths(parse_paths(INPUT));
    grid.run_steps(100);
    grid.count_black_tiles()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 3672);
    }
}
