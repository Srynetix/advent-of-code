//! Part 1

use super::{common::CalibrationDocument, INPUT};

pub fn run() -> u32 {
    CalibrationDocument::from_input(INPUT).total_calibration_value()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 54388)
    }
}
