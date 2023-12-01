//! Part 2

use super::{common::CalibrationDocument, INPUT};

pub fn run() -> u32 {
    CalibrationDocument::from_input(INPUT)
        .with_spelled_digits_converted_to_integers()
        .total_calibration_value()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 53515)
    }
}
