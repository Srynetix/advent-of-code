//! Part 2

use super::{INPUT, common::GameList};

pub fn run() -> usize {
    GameList::from_input(INPUT).sum_of_minimum_cubeset_powers()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 78_669)
    }
}
