//! Common

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

static MUL_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap());

pub struct Puzzle {
    multiplications: Vec<(u32, u32)>,
}

impl Puzzle {
    pub fn from_input(input: &str) -> Self {
        let mut multiplications = vec![];
        for capture in MUL_RGX.captures_iter(input) {
            if capture.get(1).unwrap().as_str().starts_with("mul(") {
                multiplications.push((
                    capture.get(2).unwrap().as_str().parse().unwrap(),
                    capture.get(3).unwrap().as_str().parse().unwrap(),
                ));
            }
        }

        Self { multiplications }
    }

    pub fn from_input_with_conditionals(input: &str) -> Self {
        let mut multiplications = vec![];
        let mut read_mul = true;
        for capture in MUL_RGX.captures_iter(input) {
            let opcode = capture.get(1).unwrap().as_str();
            if opcode == "do()" {
                read_mul = true;
            } else if opcode == "don't()" {
                read_mul = false;
            } else if read_mul {
                multiplications.push((
                    capture.get(2).unwrap().as_str().parse().unwrap(),
                    capture.get(3).unwrap().as_str().parse().unwrap(),
                ));
            }
        }

        Self { multiplications }
    }

    pub fn compute_multiplications(&self) -> u32 {
        self.multiplications.iter().map(|(x, y)| x * y).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;

    #[test]
    fn sample() {
        let puzzle = Puzzle::from_input(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(
            puzzle.multiplications,
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        );

        assert_eq!(puzzle.compute_multiplications(), 161);
    }

    #[test]
    fn sample_with_conditionals() {
        let puzzle = Puzzle::from_input_with_conditionals(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(puzzle.multiplications, vec![(2, 4), (8, 5)]);

        assert_eq!(puzzle.compute_multiplications(), 48);
    }
}
