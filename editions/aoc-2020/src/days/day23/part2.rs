//! Part 2

use super::{common::{run_steps, prepare_million_cups}, INPUT};

pub fn run() -> usize {
    let mut cups = prepare_million_cups(INPUT);
    run_steps(&mut cups, 10_000_000);

    let a = cups.next(1);
    let b = cups.next(a);
    a * b
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 170_836_011_000)
    }
}
