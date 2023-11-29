//! Part 2

use aoc_sx::algo::parse::parse_str_lines;

use super::{
    common::{LineCollisioner, LineParser},
    INPUT,
};

pub fn run() -> u32 {
    let lines = LineParser::parse_lines(&parse_str_lines(INPUT));
    let collisions = LineCollisioner::scan_line_intersections(&lines);
    LineCollisioner::count_overlaps(collisions)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 21_101)
    }
}
