//! Part 1

use super::{common::DataStream, INPUT};

pub fn run() -> usize {
    DataStream::from_input(INPUT).start_of_packet_marker(4)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1640);
    }
}
