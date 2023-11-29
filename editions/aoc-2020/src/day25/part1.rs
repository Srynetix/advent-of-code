//! Part 1

use super::{
    common::{determine_loop_size, parse_keys, transform_subject_number_loop},
    INPUT,
};

pub fn run() -> usize {
    let (card_key, door_key) = parse_keys(INPUT);
    let (card_ls, door_ls) = (determine_loop_size(card_key), determine_loop_size(door_key));
    let (card_ec, door_ec) = (
        transform_subject_number_loop(card_key, door_ls),
        transform_subject_number_loop(door_key, card_ls),
    );
    assert_eq!(card_ec, door_ec);

    card_ec
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 19_414_467);
    }
}
