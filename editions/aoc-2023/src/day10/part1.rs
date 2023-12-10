//! Part 1

use super::{common::PipeMaze, INPUT};

pub fn run() -> usize {
    PipeMaze::from_input(INPUT).get_longest_position_in_loop()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 6_725)
    }
}
