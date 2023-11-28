//! Part 2

use super::{common::Conway4D, INPUT};

pub fn run() -> usize {
    let mut game = Conway4D::from(INPUT);
    game.run_steps(6);
    game.count_active_cells()
}


#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 1380);
    }
}
