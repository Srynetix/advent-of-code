//! Part 1

use super::{
    common::{Interpreter, StepOutput},
    INPUT,
};

pub fn run() -> usize {
    if let StepOutput::LoopFound(e) = Interpreter::new_from_code(INPUT).run() {
        e as usize
    } else {
        panic!("Code should loop");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1930);
    }
}
