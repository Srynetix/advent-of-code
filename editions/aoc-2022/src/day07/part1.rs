//! Part 1

use super::{common::ShellSession, INPUT};

pub fn run() -> usize {
    ShellSession::from_input(INPUT).sum_directories_total_size_less_than(100_000)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1_432_936)
    }
}
