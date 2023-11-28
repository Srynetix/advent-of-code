//! Part 2

use super::{common::ExpressionParser, INPUT};

pub fn run() -> usize {
    INPUT
        .trim()
        .lines()
        .map(|l| {
            let precedences = ExpressionParser::addition_token_precedences();
            ExpressionParser::parse_and_compute_expression(l.trim(), &precedences)
        })
        .sum::<isize>() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 216_975_281_211_165);
    }
}
