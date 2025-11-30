//! Part 2

use super::{INPUT, common::OasisReport};

pub fn run() -> i32 {
    OasisReport::from_input(INPUT).sum_previous_predicted_values()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1_066)
    }
}
