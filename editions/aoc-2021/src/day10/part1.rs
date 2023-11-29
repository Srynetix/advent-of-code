//! Part 1

use aoc_sx::{algo::parse::parse_str_lines, tap::Pipe};

use super::{common::NavParser, INPUT};

pub fn run() -> u32 {
    NavParser::check_errors_on_lines(&parse_str_lines(INPUT))
        .pipe(|x| NavParser::count_errors_score(&x))
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 268_845)
    }
}
