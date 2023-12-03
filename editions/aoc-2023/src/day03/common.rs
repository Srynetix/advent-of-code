//! Common

use aoc_sx::{algo::math::Vec2, itertools::Itertools, once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicSymbol {
    value: char,
    position: Vec2,
}

impl SchematicSymbol {
    pub fn new(value: char, position: Vec2) -> Self {
        Self { value, position }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchematicNumber {
    value: u32,
    position: Vec2,
}

impl SchematicNumber {
    pub fn new(value: u32, position: Vec2) -> Self {
        Self { value, position }
    }

    pub fn count_digits(&self) -> usize {
        let mut counter = 0;
        let mut cursor = self.value;

        while cursor > 0 {
            cursor /= 10;
            counter += 1;
        }

        counter
    }

    pub fn contains_position(&self, pos: Vec2) -> bool {
        for offset in 0..self.count_digits() {
            let digit_pos = Vec2 {
                x: self.position.x + offset as isize,
                y: self.position.y,
            };

            if digit_pos == pos {
                return true;
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct Schematic {
    width: usize,
    height: usize,
    numbers: Vec<SchematicNumber>,
    symbols: Vec<SchematicSymbol>,
}

impl Schematic {
    pub fn from_input(input: &str) -> Self {
        static NUM_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?m)[0-9]+"#).unwrap());
        static SYM_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[^0-9\.\n]"#).unwrap());

        let input = input.trim();
        let width = input.find('\n').unwrap();
        let height = input.len() / width;

        let input = input.replace('\n', "");

        let numbers = NUM_RGX
            .captures_iter(&input)
            .map(|c| {
                let mat = c.get(0).unwrap();
                SchematicNumber::new(
                    mat.as_str().parse::<u32>().unwrap(),
                    Vec2 {
                        x: (mat.start() % width) as isize,
                        y: (mat.start() / width) as isize,
                    },
                )
            })
            .collect_vec();

        let symbols = SYM_RGX
            .captures_iter(&input)
            .map(|c| {
                let mat = c.get(0).unwrap();
                SchematicSymbol::new(
                    mat.as_str().chars().next().unwrap(),
                    Vec2 {
                        x: (mat.start() % width) as isize,
                        y: (mat.start() / width) as isize,
                    },
                )
            })
            .collect_vec();

        Self {
            width,
            height,
            numbers,
            symbols,
        }
    }

    pub fn numbers_with_adjacent_symbols(&self) -> impl Iterator<Item = &SchematicNumber> {
        self.numbers.iter().filter(|n| self.has_adjacent_symbols(n))
    }

    pub fn sum_of_part_numbers(&self) -> u32 {
        self.numbers_with_adjacent_symbols().map(|n| n.value).sum()
    }

    pub fn sum_of_gear_ratios(&self) -> u32 {
        self.symbols
            .iter()
            .filter(|&s| s.value == '*')
            .map(|s| self.get_symbol_adjacent_numbers(s))
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums.iter().map(|n| n.value).product::<u32>())
            .sum()
    }

    fn has_adjacent_symbols(&self, number: &SchematicNumber) -> bool {
        let num_len = number.count_digits();
        for offset in 0..num_len {
            let source_pos = Vec2 {
                x: (number.position.x + offset as isize),
                y: number.position.y,
            };

            for target_pos in self.list_adjacent_positions(source_pos) {
                // Ignore edges
                if target_pos.x < 0
                    || target_pos.x >= self.width as isize
                    || target_pos.y < 0
                    || target_pos.y >= self.height as isize
                {
                    continue;
                }

                if self.symbol_at_position(target_pos).is_some() {
                    return true;
                }
            }
        }

        false
    }

    fn get_symbol_adjacent_numbers(&self, symbol: &SchematicSymbol) -> Vec<&SchematicNumber> {
        let mut numbers = vec![];

        for target_pos in self.list_adjacent_positions(symbol.position) {
            // Ignore edges
            if target_pos.x < 0
                || target_pos.x >= self.width as isize
                || target_pos.y < 0
                || target_pos.y >= self.height as isize
            {
                continue;
            }

            if let Some(n) = self.number_at_position(target_pos) {
                if !numbers.contains(&n) {
                    numbers.push(n);
                }
            }
        }

        numbers
    }

    fn number_at_position(&self, pos: Vec2) -> Option<&SchematicNumber> {
        self.numbers.iter().find(|&n| n.contains_position(pos))
    }

    fn symbol_at_position(&self, pos: Vec2) -> Option<&SchematicSymbol> {
        self.symbols.iter().find(|&s| s.position == pos)
    }

    fn list_adjacent_positions(&self, pos: Vec2) -> [Vec2; 8] {
        [
            Vec2 {
                x: pos.x - 1,
                y: pos.y,
            },
            Vec2 {
                x: pos.x - 1,
                y: pos.y - 1,
            },
            Vec2 {
                x: pos.x,
                y: pos.y - 1,
            },
            Vec2 {
                x: pos.x + 1,
                y: pos.y - 1,
            },
            Vec2 {
                x: pos.x + 1,
                y: pos.y,
            },
            Vec2 {
                x: pos.x + 1,
                y: pos.y + 1,
            },
            Vec2 {
                x: pos.x,
                y: pos.y + 1,
            },
            Vec2 {
                x: pos.x - 1,
                y: pos.y + 1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::Schematic;

    const SAMPLE: &str = indoc! {r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "#};

    #[test]
    fn schematic() {
        let sch = Schematic::from_input(SAMPLE);
        assert_eq!(sch.sum_of_part_numbers(), 4361);
    }

    #[test]
    fn gear_ratios() {
        let sch = Schematic::from_input(SAMPLE);
        assert_eq!(sch.sum_of_gear_ratios(), 467_835);
    }
}
