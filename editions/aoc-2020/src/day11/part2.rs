//! Part 2

use super::{common::SeatLayout, INPUT};

pub fn run() -> usize {
    SeatLayout::from_input(INPUT)
        .run_with_visibility_until_stable()
        .occupied_seats
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2076);
    }
}
