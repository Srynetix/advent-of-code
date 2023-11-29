//! Common

pub struct BingoParser;
pub struct BingoPlayer;

#[derive(Debug)]
pub struct BingoPlay(Vec<u32>);

#[derive(Debug)]
pub struct BingoGrid {
    width: usize,
    height: usize,
    data: Vec<u32>,
    found: Vec<bool>,
    won: bool,
}

impl BingoGrid {
    pub fn process_number(&mut self, number: u32) -> bool {
        if let Some(pos) = self.data.iter().position(|&x| x == number) {
            self.found[pos] = true;
            true
        } else {
            false
        }
    }

    pub fn has_won(&self) -> bool {
        for row_idx in 0..self.height {
            if self.has_complete_row(row_idx) {
                return true;
            }
        }

        for col_idx in 0..self.width {
            if self.has_complete_column(col_idx) {
                return true;
            }
        }

        false
    }

    fn has_complete_row(&self, row_idx: usize) -> bool {
        for col_idx in 0..self.width {
            let value = self.found[row_idx * self.width + col_idx];
            if !value {
                return false;
            }
        }

        true
    }

    fn has_complete_column(&self, col_idx: usize) -> bool {
        for row_idx in 0..self.height {
            let value = self.found[row_idx * self.width + col_idx];
            if !value {
                return false;
            }
        }

        true
    }

    pub fn get_unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for idx in 0..self.found.len() {
            if !self.found[idx] {
                sum += self.data[idx];
            }
        }

        sum
    }
}

impl From<&str> for BingoPlay {
    fn from(value: &str) -> Self {
        let numbers: Vec<u32> = value.split(',').flat_map(|n| n.parse().ok()).collect();
        Self(numbers)
    }
}

impl From<&[&str]> for BingoGrid {
    fn from(value: &[&str]) -> Self {
        let height = value.len();
        let grid: Vec<u32> = value
            .iter()
            .flat_map(|n| n.split_whitespace().flat_map(|n| n.parse().ok()))
            .collect();
        Self {
            width: grid.len() / height,
            height,
            found: vec![false; grid.len()],
            data: grid,
            won: false,
        }
    }
}

impl BingoParser {
    pub fn parse_play_and_grids(input: &str) -> (BingoPlay, Vec<BingoGrid>) {
        let data: Vec<Vec<&str>> = input.split("\n\n").map(|l| l.lines().collect()).collect();
        let play = BingoPlay::from(data[0][0]);
        let grids = data
            .iter()
            .skip(1)
            .map(|n| BingoGrid::from(n.as_ref()))
            .collect();

        (play, grids)
    }
}

impl BingoPlayer {
    pub fn play(play: BingoPlay, mut grids: Vec<BingoGrid>) -> u32 {
        for num in play.0 {
            for grid in grids.iter_mut() {
                grid.process_number(num);
                if grid.has_won() {
                    // Compute unmarked numbers sum, times the current number
                    return grid.get_unmarked_sum() * num;
                }
            }
        }

        panic!("No grid won.")
    }

    pub fn play_waiting_for_last(play: BingoPlay, mut grids: Vec<BingoGrid>) -> u32 {
        let mut wins = 0;
        let count = grids.len();
        for num in play.0 {
            for grid in grids.iter_mut() {
                if grid.won {
                    continue;
                }

                grid.process_number(num);
                if grid.has_won() {
                    grid.won = true;
                    wins += 1;

                    if wins == count {
                        // OK, last one!
                        return grid.get_unmarked_sum() * num;
                    }
                }
            }
        }

        panic!("No grid won.")
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::{BingoParser, BingoPlayer};

    const SAMPLE_DATA: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
    "};

    #[test]
    fn test_sample() {
        let (play, grids) = BingoParser::parse_play_and_grids(SAMPLE_DATA);
        assert_eq!(BingoPlayer::play(play, grids).to_string(), "4512");
    }

    #[test]
    fn test_sample_last_one() {
        let (play, grids) = BingoParser::parse_play_and_grids(SAMPLE_DATA);
        assert_eq!(
            BingoPlayer::play_waiting_for_last(play, grids).to_string(),
            "1924"
        );
    }
}
