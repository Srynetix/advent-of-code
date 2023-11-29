//! Common

use std::collections::HashMap;

/// Card.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Card(usize);

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// Player.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Player(pub usize);

/// Deck.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Deck(Vec<Card>);

impl Deck {
    /// Take the first card available from the deck.
    pub fn take_card(&mut self) -> Option<Card> {
        if self.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }

    /// Add card at the end of the deck.
    ///
    /// # Arguments
    ///
    /// * `card` - Card
    fn add_card(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Check if the deck is empty.
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get deck length.
    fn len(&self) -> usize {
        self.0.len()
    }

    /// Clone `count` cards from the beginning of the deck into a new deck.
    ///
    /// # Arguments
    ///
    /// * `count` - Cards count to clone
    fn clone_until(&self, count: usize) -> Self {
        Self(self.0[..count].to_vec())
    }
}

impl std::fmt::Debug for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

/// Game step result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStepResult {
    /// Game is still running.
    Running,
    /// Game is finished, won by `Player`.
    Finished(Player),
}

/// Game memory.
#[derive(Debug, Default)]
pub struct GameMemory {
    rounds: HashMap<usize, Vec<(Deck, Deck)>>,
}

/// Parse decks and players from input string.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_decks(input: &str) -> ((Player, Deck), (Player, Deck)) {
    let mut blocks = input.trim().split("\n\n");

    (
        parse_deck(blocks.next().unwrap()),
        parse_deck(blocks.next().unwrap()),
    )
}

/// Parse one deck and player from input string.
///
/// # Arguments
///
/// * `input` - Input string
fn parse_deck(input: &str) -> (Player, Deck) {
    let mut lines = input.trim().lines();
    let player_id: usize = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Player ")
        .unwrap()
        .strip_suffix(':')
        .unwrap()
        .parse()
        .unwrap();

    let cards: Vec<Card> = lines.map(|n| Card(n.trim().parse().unwrap())).collect();

    (Player(player_id), Deck(cards))
}

/// Execute a game step.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
pub fn game_step(deck1: &mut Deck, deck2: &mut Deck) -> GameStepResult {
    if deck1.is_empty() {
        GameStepResult::Finished(Player(2))
    } else if deck2.is_empty() {
        GameStepResult::Finished(Player(1))
    } else {
        let card1 = deck1.take_card().unwrap();
        let card2 = deck2.take_card().unwrap();

        if card1 > card2 {
            deck1.add_card(card1);
            deck1.add_card(card2);
        } else {
            deck2.add_card(card2);
            deck2.add_card(card1);
        }

        GameStepResult::Running
    }
}

/// Run game until completion.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
pub fn run_game(deck1: &mut Deck, deck2: &mut Deck) -> Player {
    loop {
        if let GameStepResult::Finished(player) = game_step(deck1, deck2) {
            return player;
        }
    }
}

/// Execute a recursive game step.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
/// * `memory` - Game memory
/// * `game_number` - Game number
/// * `round_number` - Round number
pub fn recursive_game_step(
    deck1: &mut Deck,
    deck2: &mut Deck,
    memory: &mut GameMemory,
    game_number: usize,
) -> GameStepResult {
    // Check if round is already present, based on deck contents
    if !memory.rounds.is_empty() {
        let decks_set = (deck1.clone(), deck2.clone());
        if memory
            .rounds
            .get(&game_number)
            .unwrap()
            .contains(&decks_set)
        {
            return GameStepResult::Finished(Player(1));
        }
    }

    // Store current decks in memory
    memory
        .rounds
        .get_mut(&game_number)
        .unwrap()
        .push((deck1.clone(), deck2.clone()));

    if deck1.is_empty() {
        GameStepResult::Finished(Player(2))
    } else if deck2.is_empty() {
        GameStepResult::Finished(Player(1))
    } else {
        let card1 = deck1.take_card().unwrap();
        let card2 = deck2.take_card().unwrap();

        // Check for recursion
        let winning_player = {
            if deck1.len() >= card1.0 && deck2.len() >= card2.0 {
                let mut deck1_clone = deck1.clone_until(card1.0);
                let mut deck2_clone = deck2.clone_until(card2.0);
                run_recursive_game(&mut deck1_clone, &mut deck2_clone, memory, game_number + 1)
            } else {
                // Normal game
                if card1 > card2 {
                    Player(1)
                } else {
                    Player(2)
                }
            }
        };

        match winning_player {
            Player(1) => {
                deck1.add_card(card1);
                deck1.add_card(card2);
            }
            Player(2) => {
                deck2.add_card(card2);
                deck2.add_card(card1);
            }
            _ => unreachable!(),
        }

        GameStepResult::Running
    }
}

/// Run recursive game until completion.
///
/// # Arguments
///
/// * `deck1` - First player deck
/// * `deck2` - Second player deck
/// * `memory` - Game memory
/// * `game_number` - Game number
pub fn run_recursive_game(
    deck1: &mut Deck,
    deck2: &mut Deck,
    memory: &mut GameMemory,
    game_number: usize,
) -> Player {
    // Shortcuts for sub-games
    if game_number > 1 {
        let max_deck1_card = deck1.0.iter().max().unwrap();
        let max_deck2_card = deck2.0.iter().max().unwrap();

        if max_deck1_card > max_deck2_card {
            // Player 1 should win, because only he can win when infinite loop detected
            return Player(1);
        }
    }

    // Prepare rounds for game
    memory.rounds.entry(game_number).or_default();

    loop {
        if let GameStepResult::Finished(player) =
            recursive_game_step(deck1, deck2, memory, game_number)
        {
            // Remove unneeded memory
            memory.rounds.remove(&game_number);
            return player;
        }
    }
}

/// Calculate score for deck.
///
/// # Arguments
///
/// * `deck` - Deck
pub fn calculate_score(deck: &Deck) -> usize {
    let deck_size = deck.0.len();
    deck.0
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, card)| acc + (deck_size - idx) * card.0)
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10
    "};

    const INFINITE_SAMPLE: &str = indoc! {"
        Player 1:
        43
        19

        Player 2:
        2
        29
        14
    "};

    #[test]
    fn test_parse_decks() {
        let ((player1, deck1), (player2, deck2)) = parse_decks(SAMPLE);
        assert_eq!(player1, Player(1));
        assert_eq!(deck1.0, vec![Card(9), Card(2), Card(6), Card(3), Card(1)]);

        assert_eq!(player2, Player(2));
        assert_eq!(deck2.0, vec![Card(5), Card(8), Card(4), Card(7), Card(10)]);
    }

    #[test]
    fn test_game_step() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(SAMPLE);

        let result = game_step(&mut deck1, &mut deck2);
        assert_eq!(result, GameStepResult::Running);
        assert_eq!(
            deck1.0,
            vec![Card(2), Card(6), Card(3), Card(1), Card(9), Card(5)]
        );
        assert_eq!(deck2.0, vec![Card(8), Card(4), Card(7), Card(10)]);
    }

    #[test]
    fn test_run_game() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(SAMPLE);

        run_game(&mut deck1, &mut deck2);
        assert_eq!(deck1.0, vec![]);
        assert_eq!(
            deck2.0,
            vec![
                Card(3),
                Card(2),
                Card(10),
                Card(6),
                Card(8),
                Card(5),
                Card(9),
                Card(4),
                Card(7),
                Card(1)
            ]
        );

        assert_eq!(calculate_score(&deck1), 0);
        assert_eq!(calculate_score(&deck2), 306);
    }

    #[test]
    fn test_run_recursive_game_infinite() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(INFINITE_SAMPLE);
        let mut memory = GameMemory::default();

        let player = run_recursive_game(&mut deck1, &mut deck2, &mut memory, 1);
        assert_eq!(player, Player(1));
    }

    #[test]
    fn test_run_recursive_game_sample() {
        let ((_, mut deck1), (_, mut deck2)) = parse_decks(SAMPLE);
        let mut memory = GameMemory::default();

        let player = run_recursive_game(&mut deck1, &mut deck2, &mut memory, 1);
        assert_eq!(player, Player(2));
        assert_eq!(deck1.0, vec![]);
        assert_eq!(
            deck2.0,
            vec![
                Card(7),
                Card(5),
                Card(6),
                Card(2),
                Card(4),
                Card(1),
                Card(10),
                Card(8),
                Card(9),
                Card(3)
            ]
        );
    }
}
