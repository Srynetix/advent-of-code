//! Part 1

use super::{common::search_if_eq, INPUT};

pub fn run() -> usize {
    search_if_eq(INPUT, 2, 2020).into_iter().product()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 987_339);
    }
}
