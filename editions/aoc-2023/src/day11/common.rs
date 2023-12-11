//! Common

use aoc_sx::{algo::math::Vec2, itertools::Itertools};

#[derive(Debug)]
pub struct Universe {
    galaxies: Vec<Vec2>,
    width: usize,
    height: usize,
}

impl Universe {
    pub fn from_input(input: &str) -> Self {
        let lines = input.trim().lines().collect_vec();
        let width = lines[0].len();
        let height = lines.len();

        let galaxies = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Vec2::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self {
            galaxies,
            width,
            height,
        }
    }

    pub fn expand(self, factor: usize) -> Self {
        // Adjust factor
        let factor = (factor - 1).max(1);

        let mut current_expansion_y = 0;
        let mut new_width = self.width;
        let mut new_galaxies = vec![];

        for y in 0..self.height {
            let mut current_expansion_x = 0;

            // Y expansion
            if self.position_should_expand_y(y as isize) {
                current_expansion_y += factor;
            }

            for x in 0..self.width {
                // X expansion
                if self.position_should_expand_x(x as isize) {
                    current_expansion_x += factor;
                }

                let vec = Vec2::new(x as isize, y as isize);
                if self.galaxies.contains(&vec) {
                    let new_vec = Vec2::new(
                        vec.x + current_expansion_x as isize,
                        vec.y + current_expansion_y as isize,
                    );
                    new_galaxies.push(new_vec);
                }
            }

            new_width = self.width + current_expansion_x;
        }

        Self {
            width: new_width,
            height: self.height + current_expansion_y,
            galaxies: new_galaxies,
        }
    }

    fn position_should_expand_x(&self, x: isize) -> bool {
        self.iter_aligned_galaxies_x(x).count() == 0
    }

    fn position_should_expand_y(&self, y: isize) -> bool {
        self.iter_aligned_galaxies_y(y).count() == 0
    }

    fn iter_aligned_galaxies_x(&self, x: isize) -> impl Iterator<Item = Vec2> + '_ {
        self.galaxies
            .iter()
            .filter(move |galaxy_position| galaxy_position.x == x)
            .copied()
    }

    fn iter_aligned_galaxies_y(&self, y: isize) -> impl Iterator<Item = Vec2> + '_ {
        self.galaxies
            .iter()
            .filter(move |galaxy_position| galaxy_position.y == y)
            .copied()
    }

    fn compute_shortest_path_length(&self, source: Vec2, target: Vec2) -> usize {
        source.manhattan_distance(target) as usize
    }

    pub fn sum_shortest_paths(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|p| self.compute_shortest_path_length(*p[0], *p[1]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::Universe;

    const SAMPLE: &str = indoc! {r#"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "#};

    #[test]
    fn test() {
        let universe = Universe::from_input(SAMPLE).expand(1);
        assert_eq!(universe.sum_shortest_paths(), 374);
    }

    #[test]
    fn test_expand() {
        let universe = Universe::from_input(SAMPLE).expand(10);
        assert_eq!(universe.sum_shortest_paths(), 1030);

        let universe = Universe::from_input(SAMPLE).expand(100);
        assert_eq!(universe.sum_shortest_paths(), 8410);
    }
}
