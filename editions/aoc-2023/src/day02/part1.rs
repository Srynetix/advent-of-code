//! Part 1

use super::{
    common::{Bag, GameList},
    INPUT,
};

pub fn run() -> usize {
    let game_list = GameList::from_input(INPUT);
    let bag = Bag::from_input("12 red, 13 green, 14 blue");

    game_list.sum_of_possible_games_ids(&bag)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2283)
    }
}
