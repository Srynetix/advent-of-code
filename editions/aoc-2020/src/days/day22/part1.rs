//! Part 1

use super::{common::{parse_decks, run_game, Player, calculate_score}, INPUT};

pub fn run() -> usize {
    let ((_, mut deck1), (_, mut deck2)) = parse_decks(INPUT);
    match run_game(&mut deck1, &mut deck2) {
        Player(1) => calculate_score(&deck1),
        Player(2) => calculate_score(&deck2),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 31754)
    }
}
