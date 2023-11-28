//! Part 2

use super::{
    common::{calculate_score, parse_decks, run_recursive_game, GameMemory, Player},
    INPUT,
};

pub fn run() -> usize {
    let ((_, mut deck1), (_, mut deck2)) = parse_decks(INPUT);
    let mut memory = GameMemory::default();
    match run_recursive_game(&mut deck1, &mut deck2, &mut memory, 1) {
        Player(1) => calculate_score(&deck1),
        Player(2) => calculate_score(&deck2),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 35436)
    }
}
