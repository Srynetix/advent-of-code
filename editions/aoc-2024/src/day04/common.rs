//! Common

use std::fmt::Display;

use aoc_sx::itertools::Itertools;

#[derive(Debug)]
pub struct SquareWindow {
    chars: Vec<char>,
    size: u32,
}

impl SquareWindow {
    pub fn has_mas(&self) -> bool {
        assert_eq!(self.size, 3);
        let chars = &self.chars;

        (chars[0] == 'S'
            && chars[4] == 'A'
            && chars[8] == 'M'
            && chars[2] == 'S'
            && chars[6] == 'M')
            || (chars[0] == 'S'
                && chars[4] == 'A'
                && chars[8] == 'M'
                && chars[2] == 'M'
                && chars[6] == 'S')
            || (chars[0] == 'M'
                && chars[4] == 'A'
                && chars[8] == 'S'
                && chars[2] == 'S'
                && chars[6] == 'M')
            || (chars[0] == 'M'
                && chars[4] == 'A'
                && chars[8] == 'S'
                && chars[2] == 'M'
                && chars[6] == 'S')
    }

    pub fn iter_horizontal_window(&self) -> Vec<Vec<char>> {
        let mut output = vec![];
        let mut current = vec![];

        for y in 0..self.size {
            for x in 0..self.size {
                let c = self.chars[(x + y * self.size) as usize];
                current.push(c);
            }

            output.push(current.clone());
            current.clear();
        }

        output
    }

    pub fn iter_vertical_window(&self) -> Vec<Vec<char>> {
        let mut output = vec![];
        let mut current = vec![];

        for x in 0..self.size {
            for y in 0..self.size {
                let c = self.chars[(x + y * self.size) as usize];
                current.push(c);
            }

            output.push(current.clone());
            current.clear();
        }

        output
    }

    pub fn iter_diagonal_window_from_bottom_left(&self) -> Vec<Vec<char>> {
        fn build_diagonal(window: &SquareWindow, origin_x: u32, origin_y: u32) -> Vec<char> {
            let mut cursor_x = origin_x;
            let mut cursor_y = origin_y;
            let mut diag = vec![];

            while cursor_x < window.size && cursor_y < window.size {
                let pos = cursor_x + cursor_y * window.size;
                diag.push(window.chars[pos as usize]);

                cursor_x += 1;
                cursor_y += 1;
            }

            diag
        }

        let mut diagonals: Vec<Vec<char>> = vec![];

        for y in 0..self.size {
            diagonals.push(build_diagonal(self, 0, self.size - y - 1));
        }

        for x in 1..self.size {
            diagonals.push(build_diagonal(self, x, 0));
        }

        diagonals
    }

    pub fn iter_diagonal_window_from_bottom_right(&self) -> Vec<Vec<char>> {
        fn build_diagonal(window: &SquareWindow, origin_x: u32, origin_y: u32) -> Vec<char> {
            let mut cursor_x = origin_x;
            let mut cursor_y = origin_y as i32;
            let mut diag = vec![];

            while cursor_x < window.size && cursor_y >= 0 {
                let pos = cursor_x + (cursor_y as u32) * window.size;
                diag.push(window.chars[pos as usize]);

                cursor_x += 1;
                cursor_y -= 1;
            }

            diag
        }

        let mut diagonals: Vec<Vec<char>> = vec![];

        for x in 0..self.size {
            diagonals.push(build_diagonal(self, self.size - x - 1, self.size - 1));
        }

        for y in 1..self.size {
            diagonals.push(build_diagonal(self, 0, self.size - y - 1));
        }

        diagonals
    }

    fn count_word_in_lines(&self, word: &[char], lines: &[Vec<char>]) -> u32 {
        let reversed_word = word.iter().rev().cloned().collect_vec();
        let mut count = 0;
        for line in lines {
            for scan in line.windows(word.len()) {
                if scan == word {
                    count += 1;
                }

                if scan == reversed_word {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn count_word_in_window(&self, word: &str) -> u32 {
        let word_letters = word.chars().collect_vec();
        let mut count = 0;

        let horizontal_lines = self.iter_horizontal_window();
        let vertical_lines = self.iter_vertical_window();
        let diag_bottom_left_lines = self.iter_diagonal_window_from_bottom_left();
        let diag_bottom_right_lines = self.iter_diagonal_window_from_bottom_right();

        count += self.count_word_in_lines(&word_letters, &horizontal_lines);
        count += self.count_word_in_lines(&word_letters, &vertical_lines);
        count += self.count_word_in_lines(&word_letters, &diag_bottom_left_lines);
        count += self.count_word_in_lines(&word_letters, &diag_bottom_right_lines);
        count
    }
}

impl Display for SquareWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                write!(f, "{}", self.chars[(x + y * self.size) as usize])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct Puzzle {
    letters: Vec<char>,
    width: u32,
    height: u32,
}

impl Puzzle {
    pub fn from_input(input: &str) -> Self {
        let lines = input.lines().count();
        let first_line_length = input.lines().next().unwrap().chars().count();

        Self {
            letters: input.chars().filter(|c| *c != '\n').collect(),
            width: first_line_length as u32,
            height: lines as u32,
        }
    }

    pub fn iter_square_window(&self, size: u32) -> Vec<SquareWindow> {
        let mut output = vec![];
        let mut current = vec![];

        let mut x = 0;
        let mut y = 0;

        let mut xi = 0;
        let mut yi = 0;

        loop {
            let c = self.letters[(x + y * self.width) as usize];
            current.push(c);

            x += 1;
            xi += 1;
            if xi == size {
                y += 1;
                yi += 1;
                x -= size;
                xi = 0;

                if yi == size {
                    x += 1;
                    y -= size;
                    yi = 0;

                    // Done
                    output.push(SquareWindow {
                        chars: current.clone(),
                        size,
                    });
                    current.clear();

                    // Check if right block is out of bounds
                    if x + size > self.width {
                        x = 0;
                        y += 1;
                    }

                    if y + size > self.height {
                        break;
                    }
                }
            }
        }

        output
    }

    pub fn to_square_window(&self) -> SquareWindow {
        assert_eq!(self.width, self.height);

        SquareWindow {
            chars: self.letters.clone(),
            size: self.width,
        }
    }

    pub fn count_x_mas(&self) -> u32 {
        self.iter_square_window(3)
            .into_iter()
            .map(|w| w.has_mas() as u32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::Puzzle;

    fn sample() -> Puzzle {
        Puzzle::from_input(indoc! {r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "#})
    }

    #[test]
    fn sample_horizontal() {
        let sample = sample();
        assert_eq!(sample.to_square_window().count_word_in_window("XMAS"), 18);
    }

    #[test]
    fn sample_3x3() {
        let sample = sample();
        assert_eq!(sample.count_x_mas(), 9);
    }
}
