//! Part 1

use super::{common::TransparentPaper, INPUT};

pub fn run() -> usize {
    let paper = TransparentPaper::from(INPUT);
    let paper = paper.fold_next_rule().unwrap();
    paper.count_dots()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 655)
    }
}
