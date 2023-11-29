//! Part 2

use super::{common::Grid, INPUT};

pub fn run() -> usize {
    let mut grid = Grid::from(INPUT);
    grid.step_until_all_flashing()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 222)
    }
}
