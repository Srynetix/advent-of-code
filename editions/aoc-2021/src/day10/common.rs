//! Common

use std::collections::{HashMap, LinkedList};

use aoc_sx::itertools::Itertools;
use aoc_sx::maplit::hashmap;
use aoc_sx::once_cell::sync::Lazy;

static TOKEN_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    hashmap! {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
    }
});

static INVERTED_TOKEN_MAP: Lazy<HashMap<char, char>> =
    Lazy::new(|| TOKEN_MAP.iter().map(|(&x, &y)| (y, x)).collect());

static TOKEN_ERROR_VALUE: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    hashmap! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    }
});

static TOKEN_AUTOCOMPLETE_VALUE: Lazy<HashMap<char, u64>> = Lazy::new(|| {
    hashmap! {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
    }
});

pub struct NavParser;

impl NavParser {
    fn get_opening_token(closing_token: char) -> char {
        INVERTED_TOKEN_MAP[&closing_token]
    }

    fn get_closing_token(opening_token: char) -> char {
        TOKEN_MAP[&opening_token]
    }

    pub fn autocomplete_line(line: &str) -> String {
        let mut stack: LinkedList<char> = LinkedList::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_back(c);
                }
                ')' | ']' | '}' | '>' => {
                    stack.pop_back();
                }
                _ => unreachable!(),
            }
        }

        // Remaining characters
        let mut output = String::with_capacity(stack.len());
        while let Some(c) = stack.pop_back() {
            output.push(Self::get_closing_token(c));
        }

        output
    }

    fn check_errors_on_line(line: &str) -> Option<char> {
        let mut stack: LinkedList<char> = LinkedList::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_back(c);
                }
                ')' | ']' | '}' | '>' => match stack.pop_back() {
                    Some(c2) => {
                        if c2 != Self::get_opening_token(c) {
                            return Some(c);
                        }
                    }
                    None => return Some(c),
                },
                _ => unreachable!(),
            }
        }

        None
    }

    pub fn check_errors_on_lines(lines: &[&str]) -> Vec<char> {
        lines
            .iter()
            .flat_map(|&x| Self::check_errors_on_line(x))
            .collect()
    }

    pub fn filter_incomplete_lines<'a>(lines: &[&'a str]) -> Vec<&'a str> {
        lines
            .iter()
            .filter(|&&x| Self::check_errors_on_line(x).is_none())
            .copied()
            .collect()
    }

    pub fn extract_middle_score(scores: Vec<u64>) -> u64 {
        let len = scores.len();
        let middle = len / 2;
        scores.iter().sorted().copied().nth(middle).unwrap()
    }

    pub fn count_autocomplete_score(completed: &str) -> u64 {
        completed
            .chars()
            .fold(0, |acc, x| acc * 5 + TOKEN_AUTOCOMPLETE_VALUE[&x])
    }

    pub fn count_errors_score(errors: &[char]) -> u32 {
        errors.iter().map(|x| TOKEN_ERROR_VALUE[x]).sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::tap::Pipe;

    use super::*;

    const SAMPLE_DATA: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_sample_lines() {
        assert_eq!(
            NavParser::check_errors_on_line("{([(<{}[<>[]}>{[]{[(<()>"),
            Some('}')
        );
        assert_eq!(
            NavParser::check_errors_on_line("[[<[([]))<([[{}[[()]]]"),
            Some(')')
        );
        assert_eq!(
            NavParser::check_errors_on_line("[{[{({}]{}}([{[{{{}}([]"),
            Some(']')
        );
        assert_eq!(
            NavParser::check_errors_on_line("[<(<(<(<{}))><([]([]()"),
            Some(')')
        );
        assert_eq!(
            NavParser::check_errors_on_line("<{([([[(<>()){}]>(<<{{"),
            Some('>')
        );
    }

    #[test]
    fn test_sample_score() {
        assert_eq!(
            NavParser::check_errors_on_lines(SAMPLE_DATA)
                .pipe(|x| NavParser::count_errors_score(&x)),
            26397
        );
    }

    #[test]
    fn test_sample_autocomplete() {
        assert_eq!(
            NavParser::autocomplete_line("[({(<(())[]>[[{[]{<()<>>"),
            "}}]])})]"
        );
        assert_eq!(
            NavParser::autocomplete_line("[(()[<>])]({[<{<<[]>>("),
            ")}>]})"
        );
        assert_eq!(
            NavParser::autocomplete_line("(((({<>}<{<{<>}{[]{[]{}"),
            "}}>}>))))"
        );
        assert_eq!(
            NavParser::autocomplete_line("{<[[]]>}<{[{[{[]{()[[[]"),
            "]]}}]}]}>"
        );
        assert_eq!(
            NavParser::autocomplete_line("<{([{{}}[<[[[<>{}]]]>[]]"),
            "])}>"
        );
    }

    #[test]
    fn test_sample_autocomplete_score() {
        assert_eq!(
            NavParser::filter_incomplete_lines(SAMPLE_DATA)
                .into_iter()
                .map(NavParser::autocomplete_line)
                .map(|line| NavParser::count_autocomplete_score(&line))
                .collect::<Vec<_>>()
                .pipe(NavParser::extract_middle_score),
            288957
        );
    }
}
