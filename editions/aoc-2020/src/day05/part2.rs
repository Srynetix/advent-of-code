//! Part 2

use super::{common::BoardingPass, INPUT};

pub fn run() -> usize {
    let passes = BoardingPass::from_entries(INPUT);

    let mut seats: Vec<usize> = passes.iter().map(BoardingPass::get_seat_id).collect();
    seats.sort_unstable();

    let mut last = 0_usize;
    for s in seats {
        if last != 0 && last != s - 1 {
            return s - 1;
        }

        last = s;
    }

    panic!("Seat not found");
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 559);
    }
}
