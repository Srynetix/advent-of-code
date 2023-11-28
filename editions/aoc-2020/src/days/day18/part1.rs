//! Part 1

use super::{INPUT, common::ExpressionParser};

pub fn run() -> usize {
    INPUT
        .trim()
        .lines()
        .map(|l| {
            let precedences = ExpressionParser::default_token_precedences();
            ExpressionParser::parse_and_compute_expression(l.trim(), &precedences)
        })
        .sum::<isize>() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 45_283_905_029_161);
    }
}
