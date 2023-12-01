//! Common

use aoc_sx::once_cell::sync::Lazy;

#[derive(Debug)]
pub struct CalibrationLine(String);

impl CalibrationLine {
    pub fn from_input(input: &str) -> Self {
        Self(input.trim().to_string())
    }

    pub fn with_spelled_digits_converted_to_integers(&self) -> Self {
        static DIGITS: Lazy<&[&'static str]> = Lazy::new(|| {
            &[
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
        });

        let mut replacements = vec![];
        for (index, value) in DIGITS.iter().enumerate() {
            for (match_index, _) in self.0.match_indices(value) {
                replacements.push((
                    match_index,
                    char::from_digit((index + 1) as u32, 10).unwrap(),
                ));
            }
        }

        // Make sure the replacements are in order!
        replacements.sort_by_key(|(k, _)| *k);

        let mut new_line = self.0.clone();
        for (offset, (match_index, repl)) in replacements.iter().enumerate() {
            new_line.insert(match_index + offset, *repl);
        }

        Self(new_line)
    }

    fn first_and_last_digit(&self) -> (u32, u32) {
        let mut digits = [None, None];

        for input in self.0.chars() {
            if let Some(d) = input.to_digit(10) {
                if digits[0].is_none() {
                    digits[0] = Some(d);
                } else {
                    digits[1] = Some(d);
                }
            }
        }

        let first_digit = digits[0].expect("not a single digit in there");
        let second_digit = digits[1].unwrap_or(first_digit);

        (first_digit, second_digit)
    }

    pub fn calibration_value(&self) -> u32 {
        let (d1, d2) = self.first_and_last_digit();

        d1 * 10 + d2
    }
}

pub struct CalibrationDocument {
    lines: Vec<CalibrationLine>,
}

impl CalibrationDocument {
    pub fn from_input(input: &str) -> Self {
        Self {
            lines: input
                .trim()
                .lines()
                .map(CalibrationLine::from_input)
                .collect(),
        }
    }

    pub fn with_spelled_digits_converted_to_integers(&self) -> Self {
        Self {
            lines: self
                .lines
                .iter()
                .map(CalibrationLine::with_spelled_digits_converted_to_integers)
                .collect(),
        }
    }

    pub fn total_calibration_value(&self) -> u32 {
        self.lines
            .iter()
            .map(CalibrationLine::calibration_value)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{CalibrationDocument, CalibrationLine};

    #[test]
    fn sample_calibration_line() {
        let line = CalibrationLine::from_input("1abc2");
        assert_eq!(line.first_and_last_digit(), (1, 2));
        assert_eq!(line.calibration_value(), 12);

        let line = CalibrationLine::from_input("pqr3stu8vwx");
        assert_eq!(line.first_and_last_digit(), (3, 8));
        assert_eq!(line.calibration_value(), 38);

        let line = CalibrationLine::from_input("a1b2c3d4e5f");
        assert_eq!(line.first_and_last_digit(), (1, 5));
        assert_eq!(line.calibration_value(), 15);

        let line = CalibrationLine::from_input("treb7uchet");
        assert_eq!(line.first_and_last_digit(), (7, 7));
        assert_eq!(line.calibration_value(), 77);
    }

    #[test]
    fn sample_total_calibration() {
        let sample = concat!("1abc2\n", "pqr3stu8vwx\n", "a1b2c3d4e5f\n", "treb7uchet\n",);

        assert_eq!(
            CalibrationDocument::from_input(sample).total_calibration_value(),
            142
        );
    }

    #[test]
    fn sample_calibration_line_with_letters() {
        let line =
            CalibrationLine::from_input("two1nine").with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "2two19nine");
        assert_eq!(line.first_and_last_digit(), (2, 9));
        assert_eq!(line.calibration_value(), 29);

        let line =
            CalibrationLine::from_input("eightwothree").with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "8eigh2two3three");
        assert_eq!(line.first_and_last_digit(), (8, 3));
        assert_eq!(line.calibration_value(), 83);

        let line = CalibrationLine::from_input("abcone2threexyz")
            .with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "abc1one23threexyz");
        assert_eq!(line.first_and_last_digit(), (1, 3));
        assert_eq!(line.calibration_value(), 13);

        let line =
            CalibrationLine::from_input("xtwone3four").with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "x2tw1one34four");
        assert_eq!(line.first_and_last_digit(), (2, 4));
        assert_eq!(line.calibration_value(), 24);

        let line = CalibrationLine::from_input("4nineeightseven2")
            .with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "49nine8eight7seven2");
        assert_eq!(line.first_and_last_digit(), (4, 2));
        assert_eq!(line.calibration_value(), 42);

        let line =
            CalibrationLine::from_input("zoneight234").with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "z1on8eight234");
        assert_eq!(line.first_and_last_digit(), (1, 4));
        assert_eq!(line.calibration_value(), 14);

        let line = CalibrationLine::from_input("7pqrstsixteen")
            .with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "7pqrst6sixteen");
        assert_eq!(line.first_and_last_digit(), (7, 6));
        assert_eq!(line.calibration_value(), 76);

        let line =
            CalibrationLine::from_input("onetwone").with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "1one2tw1one");
        assert_eq!(line.first_and_last_digit(), (1, 1));
        assert_eq!(line.calibration_value(), 11);

        let line =
            CalibrationLine::from_input("eightwone").with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "8eigh2tw1one");
        assert_eq!(line.first_and_last_digit(), (8, 1));
        assert_eq!(line.calibration_value(), 81);

        let line =
            CalibrationLine::from_input("eighthree").with_spelled_digits_converted_to_integers();
        assert_eq!(line.0, "8eigh3three");
        assert_eq!(line.first_and_last_digit(), (8, 3));
        assert_eq!(line.calibration_value(), 83);
    }

    #[test]
    fn sample_total_calibration_with_letters() {
        let sample = concat!(
            "two1nine\n",
            "eightwothree\n",
            "abcone2threexyz\n",
            "xtwone3four\n",
            "4nineeightseven2\n",
            "zoneight234\n",
            "7pqrstsixteen"
        );

        assert_eq!(
            CalibrationDocument::from_input(sample)
                .with_spelled_digits_converted_to_integers()
                .total_calibration_value(),
            281
        );
    }
}
