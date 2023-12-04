//! Common

use std::{collections::HashMap, ops::Deref};

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Blue,
    Red,
    Green,
}

impl Color {
    pub fn from_name(name: &str) -> Self {
        match name {
            "blue" => Self::Blue,
            "red" => Self::Red,
            "green" => Self::Green,
            _ => panic!("Unsupported color {name}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CubeSet(HashMap<Color, usize>);

impl CubeSet {
    pub fn from_input(input: &str) -> Self {
        Self(
            input
                .split(", ")
                .map(|l| {
                    let mut it = l.split_whitespace();
                    let quantity = it.next().unwrap().parse::<usize>().unwrap();
                    let color = Color::from_name(it.next().unwrap());

                    (color, quantity)
                })
                .collect(),
        )
    }

    pub fn power(&self) -> usize {
        self.0.values().product()
    }
}

impl Deref for CubeSet {
    type Target = HashMap<Color, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

impl Game {
    pub fn from_input(input: &str) -> Self {
        static RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Game (\d+): (.*)"#).unwrap());

        let capture = RGX.captures(input.trim()).unwrap();
        let game_id = capture[1].parse::<usize>().unwrap();
        let cube_sets = &capture[2];

        Self {
            id: game_id,
            sets: cube_sets.split("; ").map(CubeSet::from_input).collect(),
        }
    }

    pub fn can_run_game_with_cubeset(&self, cubeset: &CubeSet) -> bool {
        for set in &self.sets {
            for (color, count) in set.iter() {
                if cubeset.get(color).unwrap_or(&0) < count {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_minimum_cubeset_required(&self) -> CubeSet {
        let mut map = HashMap::<Color, usize>::new();

        for set in &self.sets {
            for (color, count) in set.iter() {
                map.entry(*color)
                    .and_modify(|e| *e = *count.max(e))
                    .or_insert(*count);
            }
        }

        CubeSet(map)
    }
}

#[derive(Debug)]
pub struct GameList(HashMap<usize, Game>);

impl GameList {
    pub fn from_input(input: &str) -> Self {
        Self(
            input
                .trim()
                .lines()
                .map(|l| l.trim())
                .map(Game::from_input)
                .map(|g| (g.id, g))
                .collect(),
        )
    }

    pub fn filter_possible_games(&self, set: &CubeSet) -> Vec<&Game> {
        let mut games = vec![];

        for game in self.0.values() {
            if game.can_run_game_with_cubeset(set) {
                games.push(game);
            }
        }

        games
    }

    pub fn sum_of_possible_games_ids(&self, set: &CubeSet) -> usize {
        self.filter_possible_games(set).iter().map(|g| g.id).sum()
    }

    pub fn sum_of_minimum_cubeset_powers(&self) -> usize {
        self.0
            .values()
            .map(|g| g.get_minimum_cubeset_required().power())
            .sum()
    }
}

impl Deref for GameList {
    type Target = HashMap<usize, Game>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use aoc_sx::{indoc::indoc, itertools::Itertools, maplit::hashmap};

    use crate::day02::common::{CubeSet, Game};

    use super::{Color, GameList};

    const SAMPLE: &str = indoc! {r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#};

    #[test]
    fn parse_color() {
        assert_eq!(Color::from_name("red"), Color::Red);
        assert_eq!(Color::from_name("blue"), Color::Blue);
        assert_eq!(Color::from_name("green"), Color::Green);
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            CubeSet::from_input("3 blue, 4 red").deref(),
            &hashmap! {
                Color::Blue => 3,
                Color::Red => 4
            }
        );
    }

    #[test]
    fn parse_game() {
        assert_eq!(
            Game::from_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                sets: vec![
                    CubeSet(hashmap! {
                        Color::Blue => 3,
                        Color::Red => 4
                    }),
                    CubeSet(hashmap! {
                        Color::Red => 1,
                        Color::Green => 2,
                        Color::Blue => 6
                    }),
                    CubeSet(hashmap! {
                        Color::Green => 2
                    })
                ]
            }
        );
    }

    #[test]
    fn parse_game_list() {
        let game_list = GameList::from_input(SAMPLE);
        assert_eq!(game_list.len(), 5);
    }

    #[test]
    fn possible_games() {
        let game_list = GameList::from_input(SAMPLE);
        let bag = CubeSet::from_input("12 red, 13 green, 14 blue");
        let possible_games = game_list.filter_possible_games(&bag);
        assert_eq!(
            possible_games.iter().map(|g| g.id).sorted().collect_vec(),
            vec![1, 2, 5]
        );
    }

    #[test]
    fn possible_games_sum() {
        let game_list = GameList::from_input(SAMPLE);
        let bag = CubeSet::from_input("12 red, 13 green, 14 blue");
        assert_eq!(game_list.sum_of_possible_games_ids(&bag), 8);
    }

    #[test]
    fn minimum_cubeset() {
        let game = Game::from_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let expected = CubeSet(hashmap! {
            Color::Red => 4,
            Color::Green => 2,
            Color::Blue => 6
        });
        assert_eq!(&game.get_minimum_cubeset_required(), &expected);
        assert_eq!(expected.power(), 48);
    }

    #[test]
    fn minimum_cubeset_power_sum() {
        let game_list = GameList::from_input(SAMPLE);
        assert_eq!(game_list.sum_of_minimum_cubeset_powers(), 2286);
    }
}
