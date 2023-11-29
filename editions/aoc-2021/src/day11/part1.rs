//! Part 1

use aoc_sx::algo::fs::get_debug_path;

use super::{
    common::{save_grid_to_disk, Grid},
    INPUT,
};

pub fn run() -> usize {
    let mut grid = Grid::from(INPUT);
    let answer = grid.step_for(100);

    let path = get_debug_path().join("aoc2021-day11.png");
    save_grid_to_disk(&grid, &path);

    answer
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1_603)
    }
}
