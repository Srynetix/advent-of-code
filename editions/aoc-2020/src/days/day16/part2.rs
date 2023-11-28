//! Part 2

use super::{common::InputParser, INPUT};

pub fn run() -> usize {
    let input = InputParser::from(INPUT);

    input
        .map_ticket_fields()
        .iter()
        .filter_map(|(&k, &v)| {
            if k.starts_with("departure") {
                Some(input.your_ticket.numbers[v - 1])
            } else {
                None
            }
        })
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 410_460_648_673);
    }
}
