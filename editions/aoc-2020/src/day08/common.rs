//! Common

use std::collections::HashSet;

use aoc_sx::serde_plain;
use serde::Deserialize;

/// Operation code
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OpCode {
    /// No operation
    Nop,
    /// Increases or decreases the accumulator
    Acc,
    /// Jump to relative instruction number
    Jmp,
}

/// Instruction
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
    value: isize,
}

impl Instruction {
    /// Creates a new instruction.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Instruction `OpCode`
    /// * `value` - Instruction value
    pub const fn new(opcode: OpCode, value: isize) -> Self {
        Self { opcode, value }
    }

    /// Convert instruction to fixed instruction by swapping `Nop` and `Jmp` `OpCode`s.
    pub fn to_fixed_instruction(&self) -> Self {
        match self.opcode {
            OpCode::Acc => self.clone(),
            OpCode::Nop => Self::new(OpCode::Jmp, self.value),
            OpCode::Jmp => Self::new(OpCode::Nop, self.value),
        }
    }
}

/// Instruction parser
pub struct Parser;

impl Parser {
    /// Parse instruction from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_instruction(input: &str) -> Instruction {
        let mut tokens = input.split_whitespace();
        let opcode: OpCode = tokens
            .next()
            .and_then(|res| serde_plain::from_str(res).unwrap())
            .unwrap();
        let value: isize = tokens
            .next()
            .and_then(|res| serde_plain::from_str(res).unwrap())
            .unwrap();

        Instruction::new(opcode, value)
    }

    /// Parse code.
    ///
    /// # Arguments
    ///
    /// * `code` - Source code
    pub fn parse_code(code: &str) -> Vec<Instruction> {
        code.lines().map(Self::parse_instruction).collect()
    }
}

/// Step output
#[derive(Debug, Eq, PartialEq)]
pub enum StepOutput {
    /// Normal execution
    Normal,
    /// Loop found, with current accumulator value
    LoopFound(isize),
    /// Finished, with current accumulator value
    Finished(isize),
    /// Error
    Error,
}

/// Interpreter
pub struct Interpreter {
    instructions: Vec<Instruction>,
    accumulator: isize,
    cursor: usize,
    seen_instructions: HashSet<usize>,
}

impl Interpreter {
    /// Creates interpreter from code.
    ///
    /// # Arguments
    ///
    /// * `code` - Source code
    pub fn new_from_code(code: &str) -> Self {
        Self {
            instructions: Parser::parse_code(code),
            accumulator: 0,
            cursor: 0,
            seen_instructions: HashSet::new(),
        }
    }

    /// Reset state, conserving current instructions.
    pub fn reset_state(&mut self) {
        self.accumulator = 0;
        self.cursor = 0;
        self.seen_instructions = HashSet::new();
    }

    /// Step on next instruction.
    pub fn step(&mut self) -> StepOutput {
        if self.cursor >= self.instructions.len() {
            // End !
            return StepOutput::Finished(self.accumulator);
        }

        let instr = &self.instructions[self.cursor];
        let mut next_cursor = self.cursor + 1;
        match instr.opcode {
            OpCode::Acc => self.accumulator += instr.value,
            OpCode::Nop => (),
            OpCode::Jmp => next_cursor = (self.cursor as isize + instr.value) as usize,
        }

        // Update cursor and seen instructions
        self.seen_instructions.insert(self.cursor);
        self.cursor = next_cursor;

        if self.seen_instructions.contains(&self.cursor) {
            StepOutput::LoopFound(self.accumulator)
        } else {
            StepOutput::Normal
        }
    }

    /// Run interpreter.
    /// Breaks on Finished or LoopFound.
    pub fn run(&mut self) -> StepOutput {
        loop {
            match self.step() {
                StepOutput::Normal => (),
                step => {
                    return step;
                }
            }
        }
    }

    /// Run on repair mode.
    /// Breaks on Finished, or return Error.
    pub fn run_repair_mode(&mut self) -> StepOutput {
        let original_instructions = self.instructions.clone();
        let reparation_choices: Vec<usize> = original_instructions
            .iter()
            .enumerate()
            .filter_map(|(idx, ins)| {
                if ins.opcode == OpCode::Acc {
                    None
                } else {
                    Some(idx)
                }
            })
            .collect();
        let reparation_limit = reparation_choices.len();
        let mut reparation_cursor = 0;

        while reparation_cursor < reparation_limit {
            let reparation = reparation_choices[reparation_cursor];
            let mut new_instructions = original_instructions.clone();
            let instr_to_repair = &new_instructions[reparation];
            new_instructions[reparation] = instr_to_repair.to_fixed_instruction();

            // Set instructions as current and reset state
            self.instructions = new_instructions;
            self.reset_state();

            if let StepOutput::Finished(i) = self.run() {
                return StepOutput::Finished(i);
            }

            reparation_cursor += 1;
        }

        StepOutput::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE_SAMPLE: &str = r#"nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6"#;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Parser::parse_instruction("jmp +4"),
            Instruction::new(OpCode::Jmp, 4)
        );
        assert_eq!(
            Parser::parse_instruction("jmp -4"),
            Instruction::new(OpCode::Jmp, -4)
        );
        assert_eq!(
            Parser::parse_instruction("nop +0"),
            Instruction::new(OpCode::Nop, 0)
        );
    }

    #[test]
    fn test_parse_code() {
        assert_eq!(
            Parser::parse_code("jmp +4\nnop +0"),
            vec![
                Instruction::new(OpCode::Jmp, 4),
                Instruction::new(OpCode::Nop, 0)
            ]
        );
    }

    #[test]
    fn test_interpreter_run() {
        assert_eq!(
            Interpreter::new_from_code(CODE_SAMPLE).run(),
            StepOutput::LoopFound(5)
        );
    }

    #[test]
    fn test_interpreter_run_repair_mode() {
        assert_eq!(
            Interpreter::new_from_code(CODE_SAMPLE).run_repair_mode(),
            StepOutput::Finished(8)
        );
    }
}
