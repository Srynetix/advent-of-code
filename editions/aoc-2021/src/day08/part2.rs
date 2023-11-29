//! Part 2

use aoc_sx::algo::parse::parse_lines;

use super::{
    common::{find_solution, PatternLine},
    INPUT,
};

pub fn run() -> usize {
    let lines: Vec<PatternLine> = parse_lines(INPUT);
    lines
        .iter()
        .map(|line| {
            let input: Vec<_> = line.signal_patterns.iter().map(|x| &x.0[..]).collect();
            let mapping = find_solution(&input);
            line.decode_output_with_mapping(&mapping)
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 978_171)
    }
}
