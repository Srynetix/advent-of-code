//! Common

use std::collections::HashSet;

use aoc_sx::{algo::math::Vec2, itertools::Itertools};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    pub fn invert(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    pub fn to_unit_vector(&self) -> Vec2 {
        match *self {
            Self::North => Vec2::new(0, -1),
            Self::South => Vec2::new(0, 1),
            Self::East => Vec2::new(1, 0),
            Self::West => Vec2::new(-1, 0),
        }
    }

    pub fn all_directions() -> &'static [Self] {
        &[Self::North, Self::East, Self::West, Self::South]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeCell {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl PipeCell {
    pub fn from_input(input: char) -> Self {
        match input {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            other => panic!("Unknown cell {other}"),
        }
    }

    pub fn can_enter_following_direction(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => matches!(
                self,
                Self::Vertical | Self::SouthEast | Self::SouthWest | Self::Start
            ),
            Direction::South => matches!(
                self,
                Self::Vertical | Self::NorthEast | Self::NorthWest | Self::Start
            ),
            Direction::East => matches!(
                self,
                Self::Horizontal | Self::NorthWest | Self::SouthWest | Self::Start
            ),
            Direction::West => matches!(
                self,
                Self::Horizontal | Self::NorthEast | Self::SouthEast | Self::Start
            ),
        }
    }

    pub fn can_exit_following_direction(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => matches!(
                self,
                Self::Vertical | Self::NorthEast | Self::NorthWest | Self::Start
            ),
            Direction::South => matches!(
                self,
                Self::Vertical | Self::SouthEast | Self::SouthWest | Self::Start
            ),
            Direction::East => matches!(
                self,
                Self::Horizontal | Self::NorthEast | Self::SouthEast | Self::Start
            ),
            Direction::West => matches!(
                self,
                Self::Horizontal | Self::NorthWest | Self::SouthWest | Self::Start
            ),
        }
    }
}

#[derive(Debug)]
pub struct MoveOutput {
    target: Vec2,
    can_move: bool,
}

pub struct PipeMaze {
    cells: Vec<PipeCell>,
    width: usize,
    height: usize,
}

impl PipeMaze {
    pub fn from_input(input: &str) -> Self {
        let lines = input.trim().lines().collect_vec();
        let width = lines[0].len();
        let height = lines.len();

        Self {
            cells: lines
                .into_iter()
                .flat_map(|l| l.chars().map(PipeCell::from_input))
                .collect(),
            width,
            height,
        }
    }

    pub fn cell_at_position(&self, position: Vec2) -> Option<PipeCell> {
        if position.x >= (self.width as isize)
            || position.x < 0
            || position.y >= (self.height as isize)
            || position.y < 0
        {
            None
        } else {
            Some(self.cells[position.x as usize + position.y as usize * self.width])
        }
    }

    fn can_move_towards(
        &self,
        source_cell: PipeCell,
        direction: Direction,
        destination_cell: PipeCell,
    ) -> bool {
        source_cell.can_exit_following_direction(direction)
            && destination_cell.can_enter_following_direction(direction)
    }

    fn get_origin_point(&self) -> Vec2 {
        for (idx, cell) in self.cells.iter().enumerate() {
            if *cell == PipeCell::Start {
                let x = idx % self.width;
                let y = idx / self.width;
                return Vec2::new(x as isize, y as isize);
            }
        }

        panic!("Not found :(")
    }

    pub fn move_towards_direction(&self, position: Vec2, direction: Direction) -> MoveOutput {
        let target = position + direction.to_unit_vector();
        let source_cell = self.cell_at_position(position).unwrap();

        if let Some(destination_cell) = self.cell_at_position(target) {
            MoveOutput {
                target,
                can_move: self.can_move_towards(source_cell, direction, destination_cell),
            }
        } else {
            MoveOutput {
                target,
                can_move: false,
            }
        }
    }

    pub fn get_loop_path(&self) -> Vec<Vec2> {
        let origin = self.get_origin_point();
        let mut cursor = origin;
        let mut path = vec![];

        let mut seen = HashSet::new();
        seen.insert(origin);

        loop {
            let mut found_something = false;
            for direction in Direction::all_directions() {
                let move_output = self.move_towards_direction(cursor, *direction);
                if move_output.can_move {
                    if move_output.target == origin {
                        return path;
                    } else if seen.contains(&move_output.target) {
                        continue;
                    }

                    cursor = move_output.target;
                    path.push(cursor);
                    seen.insert(cursor);
                    found_something = true;
                    break;
                }
            }

            if !found_something {
                panic!("Blocked!");
            }
        }
    }

    pub fn get_longest_position_in_loop(&self) -> usize {
        self.get_loop_path().len().div_ceil(2)
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::{algo::math::Vec2, indoc::indoc};

    use super::{PipeCell, PipeMaze};

    const SAMPLE: &str = indoc! {r#"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
    "#};

    const SAMPLE2: &str = indoc! {r#"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "#};

    #[test]
    fn moves() {
        let maze = PipeMaze::from_input(SAMPLE);
        assert_eq!(
            maze.cell_at_position(Vec2::new(1, 1)),
            Some(PipeCell::Start)
        );
    }

    #[test]
    fn steps() {
        let maze = PipeMaze::from_input(SAMPLE);
        assert_eq!(maze.get_longest_position_in_loop(), 4);

        let maze = PipeMaze::from_input(SAMPLE2);
        assert_eq!(maze.get_longest_position_in_loop(), 8);
    }
}
