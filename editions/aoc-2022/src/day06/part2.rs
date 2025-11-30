//! Part 2

use super::{INPUT, common::DataStream};

pub fn run() -> usize {
    DataStream::from_input(INPUT).start_of_packet_marker(14)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 3613)
    }
}
