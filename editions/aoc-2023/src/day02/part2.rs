//! Part 2

use super::{common::GameList, INPUT};

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
