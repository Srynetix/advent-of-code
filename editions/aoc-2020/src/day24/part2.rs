//! Part 2

use super::{
    common::{parse_paths, HexGrid},
    INPUT,
};

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
