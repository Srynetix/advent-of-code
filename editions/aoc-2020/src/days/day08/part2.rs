//! Part 2

use super::{common::{StepOutput, Interpreter}, INPUT};

pub fn run() -> usize {
    if let StepOutput::Finished(e) = Interpreter::new_from_code(INPUT).run_repair_mode() {
        e as usize
    } else {
        panic!("Code should finish");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1688);
    }
}
