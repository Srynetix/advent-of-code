//! Part 2

use super::{common::ShellSession, INPUT};

pub fn run() -> usize {
    ShellSession::from_input(INPUT)
        .find_smallest_directory_to_free_space()
        .total_size()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 272_298)
    }
}
