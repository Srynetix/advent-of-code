//! Part 2

use aoc_sx::{algo::parse::parse_str_lines, tap::Pipe};

use super::{common::NavParser, INPUT};

pub fn run() -> u64 {
    NavParser::filter_incomplete_lines(&parse_str_lines(INPUT))
        .into_iter()
        .map(NavParser::autocomplete_line)
        .map(|line| NavParser::count_autocomplete_score(&line))
        .collect::<Vec<_>>()
        .pipe(NavParser::extract_middle_score)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 4_038_824_534)
    }
}
