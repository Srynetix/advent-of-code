//! Common

use std::collections::VecDeque;

const INITIAL_BUFFER_SIZE: usize = 1_048_576; // 1 MB

/// Memory game
pub struct MemoryGame {
    memory: Vec<usize>,
    input: VecDeque<usize>,
    turn: usize,
}

impl MemoryGame {
    /// Creates new game from string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_str_input(input: &str) -> Self {
        let input: Vec<_> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        Self::from_vec(input)
    }

    /// Creates new game from vec.
    ///
    /// # Arguments
    ///
    /// * `input` - Input vec
    pub fn from_vec(input: Vec<usize>) -> Self {
        Self {
            memory: vec![0; INITIAL_BUFFER_SIZE],
            input: input.into(),
            turn: 1,
        }
    }

    /// Step.
    pub fn step(&mut self) -> usize {
        let i = self.input.pop_front().unwrap_or(0);

        if i >= self.memory.len() {
            self.memory.resize(i * 2, 0);
        }

        let t = self.memory[i];
        if t > 0 {
            self.input.push_back(self.turn - t);
        }

        self.memory[i] = self.turn;
        self.turn += 1;
        i
    }

    /// Run for `n` steps.
    pub fn run_steps(&mut self, n: usize) -> usize {
        let mut result = 0;

        if n == 0 {
            panic!("You need at least one step.");
        }

        for _ in 0..n {
            result = self.step();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_sample_steps() {
        let mut game = MemoryGame::from_vec(vec![0, 3, 6]);
        assert_eq!(game.step(), 0);
        assert_eq!(game.step(), 3);
        assert_eq!(game.step(), 6);
        assert_eq!(game.step(), 0);
        assert_eq!(game.step(), 3);
        assert_eq!(game.step(), 3);
        assert_eq!(game.step(), 1);
        assert_eq!(game.step(), 0);
        assert_eq!(game.step(), 4);
        assert_eq!(game.step(), 0);
    }

    #[test]
    fn test_first_sample_run() {
        let mut game = MemoryGame::from_vec(vec![0, 3, 6]);
        assert_eq!(game.run_steps(2020), 436);
    }

    #[test]
    fn test_more_samples() {
        assert_eq!(MemoryGame::from_vec(vec![1, 3, 2]).run_steps(2020), 1);
        assert_eq!(MemoryGame::from_vec(vec![2, 1, 3]).run_steps(2020), 10);
        assert_eq!(MemoryGame::from_vec(vec![1, 2, 3]).run_steps(2020), 27);
        assert_eq!(MemoryGame::from_vec(vec![2, 3, 1]).run_steps(2020), 78);
        assert_eq!(MemoryGame::from_vec(vec![3, 2, 1]).run_steps(2020), 438);
        assert_eq!(MemoryGame::from_vec(vec![3, 1, 2]).run_steps(2020), 1836);
    }
}
