//! Common

pub struct Puzzle {
    reports: Vec<Report>,
}

impl Puzzle {
    pub fn from_input(input: &str) -> Self {
        let reports = input.lines().map(Report::from_line).collect();
        Self { reports }
    }
}

#[derive(Clone)]
pub struct Report {
    levels: Vec<Level>,
}

impl Report {
    pub fn from_line(line: &str) -> Self {
        let levels = line.split_whitespace().map(Level::from_line).collect();
        Self { levels }
    }

    pub fn without_index(&self, idx: usize) -> Self {
        let mut new_levels = self.levels.clone();
        new_levels.remove(idx);
        Self { levels: new_levels }
    }
}

#[derive(Clone, Copy)]
pub struct Level(u8);

impl Level {
    pub fn from_line(s: &str) -> Self {
        Self(s.parse::<u8>().unwrap())
    }
}

pub struct Analyzer;

impl Analyzer {
    pub fn check_report_safety_with_dampener(report: &Report) -> bool {
        if Self::check_report_safety(report) {
            return true;
        }

        // Iterate on everything
        for x in 0..report.levels.len() {
            if Self::check_report_safety(&report.without_index(x)) {
                return true;
            }
        }

        false
    }

    pub fn check_report_safety(report: &Report) -> bool {
        let mut last_item = &report.levels[0];
        let mut prev_sign = (last_item.0 as i8 - report.levels[1].0 as i8).signum();
        for item in report.levels.iter().skip(1) {
            let v1 = last_item.0 as i8;
            let v2 = item.0 as i8;

            let new_sign = (v1 - v2).signum();
            let distance = (v1 - v2).abs();

            if new_sign != prev_sign || distance == 0 || distance > 3 {
                return false;
            }

            last_item = item;
            prev_sign = new_sign;
        }

        true
    }

    pub fn count_safe_reports(puzzle: &Puzzle) -> u32 {
        puzzle
            .reports
            .iter()
            .map(|r| Self::check_report_safety(r) as u32)
            .sum()
    }

    pub fn count_safe_reports_with_dampener(puzzle: &Puzzle) -> u32 {
        puzzle
            .reports
            .iter()
            .map(|r| Self::check_report_safety_with_dampener(r) as u32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use crate::day02::common::Analyzer;

    use super::Puzzle;

    const SAMPLE: &str = indoc! { r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "#};

    #[test]
    fn sample() {
        let puzzle = Puzzle::from_input(SAMPLE);
        assert_eq!(Analyzer::check_report_safety(&puzzle.reports[0]), true);
        assert_eq!(Analyzer::check_report_safety(&puzzle.reports[3]), false);
        assert_eq!(Analyzer::count_safe_reports(&puzzle), 2);
    }

    #[test]
    fn sample_with_dampener() {
        let puzzle = Puzzle::from_input(SAMPLE);
        assert_eq!(
            Analyzer::check_report_safety_with_dampener(&puzzle.reports[0]),
            true
        );
        assert_eq!(
            Analyzer::check_report_safety_with_dampener(&puzzle.reports[1]),
            false
        );
        assert_eq!(
            Analyzer::check_report_safety_with_dampener(&puzzle.reports[2]),
            false
        );
        assert_eq!(
            Analyzer::check_report_safety_with_dampener(&puzzle.reports[3]),
            true
        );
        assert_eq!(
            Analyzer::check_report_safety_with_dampener(&puzzle.reports[4]),
            true
        );
        assert_eq!(
            Analyzer::check_report_safety_with_dampener(&puzzle.reports[5]),
            true
        );
        assert_eq!(Analyzer::count_safe_reports_with_dampener(&puzzle), 4);
    }
}
