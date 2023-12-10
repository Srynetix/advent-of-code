//! Common

#[derive(Debug)]
pub struct NumberLine(Vec<i32>);

impl NumberLine {
    pub fn from_input(input: &str) -> Self {
        Self(
            input
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        )
    }

    pub fn predict_next_value(&self) -> i32 {
        Self::predict_next_value_rec(&self.0)
    }

    fn predict_next_value_rec(values: &[i32]) -> i32 {
        let mut output = vec![];
        let mut all_0 = true;

        for chunk in values.windows(2) {
            let diff = chunk[1] - chunk[0];
            if diff != 0 {
                all_0 = false;
            }
            output.push(diff);
        }

        if all_0 {
            values[0]
        } else {
            let value = Self::predict_next_value_rec(&output);
            let last_value = values.last().unwrap();
            last_value + value
        }
    }

    pub fn predict_previous_value(&self) -> i32 {
        Self::predict_previous_value_rec(&self.0)
    }

    fn predict_previous_value_rec(values: &[i32]) -> i32 {
        let mut output = vec![];
        let mut all_0 = true;

        for chunk in values.windows(2) {
            let diff = chunk[1] - chunk[0];
            if diff != 0 {
                all_0 = false;
            }
            output.push(diff);
        }

        if all_0 {
            values[0]
        } else {
            let value = Self::predict_previous_value_rec(&output);
            let last_value = values[0];
            last_value - value
        }
    }
}

#[derive(Debug)]
pub struct OasisReport {
    lines: Vec<NumberLine>,
}

impl OasisReport {
    pub fn from_input(input: &str) -> Self {
        Self {
            lines: input.trim().lines().map(NumberLine::from_input).collect(),
        }
    }

    pub fn sum_next_predicted_values(&self) -> i32 {
        self.lines.iter().map(|l| l.predict_next_value()).sum()
    }

    pub fn sum_previous_predicted_values(&self) -> i32 {
        self.lines.iter().map(|l| l.predict_previous_value()).sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use crate::day09::common::OasisReport;

    const SAMPLE: &str = indoc! {r#"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "#};

    #[test]
    fn sample_next_value() {
        assert_eq!(
            OasisReport::from_input(SAMPLE).sum_next_predicted_values(),
            114
        );
    }

    #[test]
    fn sample_previous_value() {
        assert_eq!(
            OasisReport::from_input(SAMPLE).sum_previous_predicted_values(),
            2
        );
    }
}
