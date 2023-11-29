//! Common

use std::{collections::HashSet, path::Path};

use aoc_sx::image::ImageBuffer;

fn state_to_color(state: u32) -> &'static [u8; 3] {
    // Palette from https://coolors.co/d5bff2-d3bae3-d1b6d6-b197bd-706791-4e507a-364066-1d3057-10274f-031e47
    match state {
        9 => &[213, 191, 242],
        8 => &[211, 186, 227],
        7 => &[209, 182, 214],
        6 => &[177, 151, 189],
        5 => &[112, 103, 145],
        4 => &[78, 80, 122],
        3 => &[54, 64, 102],
        2 => &[29, 48, 87],
        1 => &[16, 39, 79],
        0 => &[3, 30, 71],
        _ => unreachable!(),
    }
}

pub fn save_grid_to_disk(map: &Grid, output: &Path) {
    let mut buffer = ImageBuffer::new(map.width as u32, map.height as u32);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let pos = map.xy_to_pos(x as usize, y as usize);
        let state = map.data[pos];
        *pixel = aoc_sx::image::Rgb(*state_to_color(state));
    }

    buffer.save(output).expect("Could not generate grid.");
}

#[derive(Clone, Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Grid {
    #[inline]
    fn pos_to_xy(&self, position: usize) -> (usize, usize) {
        (
            position % self.width,
            (position as f32 / self.width as f32) as usize,
        )
    }

    #[inline]
    fn xy_to_pos(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn get_neighbors_position(&self, position: usize) -> Vec<usize> {
        let mut neighbors_pos = Vec::new();
        let (x, y) = self.pos_to_xy(position);

        for ny in -1..=1 {
            for nx in -1..=1 {
                let cx = x as i32 + nx;
                let cy = y as i32 + ny;

                if cx < 0
                    || cy < 0
                    || cx >= self.width as i32
                    || cy >= self.height as i32
                    || (nx == 0 && ny == 0)
                {
                    continue;
                } else {
                    let pos = self.xy_to_pos(cx as usize, cy as usize);
                    neighbors_pos.push(pos);
                }
            }
        }

        neighbors_pos
    }

    pub fn step(&mut self) -> usize {
        let mut maxed = HashSet::new();
        let mut flashes = 0;

        for pos in 0..self.data.len() {
            if !maxed.contains(&pos) {
                flashes += self.increment_cell(pos, &mut maxed);
            }
        }

        flashes
    }

    pub fn increment_cell(&mut self, position: usize, maxed: &mut HashSet<usize>) -> usize {
        let mut flashes = 0;
        let mut value = self.data[position];
        value += 1;

        if value > 9 {
            maxed.insert(position);
            flashes += 1;
            self.data[position] = 0;
            for neighbor in self.get_neighbors_position(position) {
                if !maxed.contains(&neighbor) {
                    flashes += self.increment_cell(neighbor, maxed);
                }
            }
        } else {
            self.data[position] = value;
        }

        flashes
    }

    pub fn step_for(&mut self, count: usize) -> usize {
        let mut flashes = 0;

        for _ in 0..count {
            flashes += self.step();
        }

        flashes
    }

    pub fn step_until_all_flashing(&mut self) -> usize {
        let target = self.width * self.height;
        let mut i = 0;

        loop {
            i += 1;

            if self.step() == target {
                return i;
            }
        }
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);

        for line in lines {
            for c in line.chars() {
                data.push(c.to_digit(10).expect("Char should be a digit"));
            }
        }

        Self {
            width,
            height,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::{algo::fs::get_debug_path, indoc::indoc};

    use super::{save_grid_to_disk, Grid};

    const SMALL_SAMPLE_DATA: &str = indoc! {"
        11111
        19991
        19191
        19991
        11111
    "};

    const SAMPLE_DATA: &str = indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn test_small_sample() {
        let mut grid = Grid::from(SMALL_SAMPLE_DATA);
        assert_eq!(grid.step(), 9);
        assert_eq!(
            grid.data,
            [3, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3, 4, 5, 4, 3]
        );

        let path = get_debug_path().join("aoc2021-day11-small-sample.png");
        save_grid_to_disk(&grid, &path);

        assert_eq!(grid.step(), 0);
        assert_eq!(
            grid.data,
            [4, 5, 6, 5, 4, 5, 1, 1, 1, 5, 6, 1, 1, 1, 6, 5, 1, 1, 1, 5, 4, 5, 6, 5, 4]
        );
    }

    #[test]
    fn test_sample() {
        let mut grid = Grid::from(SAMPLE_DATA);
        assert_eq!(grid.step_for(10), 204);
        assert_eq!(grid.step_for(90), 1656 - 204);

        let path = get_debug_path().join("aoc2021-day11-sample.png");
        save_grid_to_disk(&grid, &path);
    }

    #[test]
    fn test_sample_all_flashing() {
        let mut grid = Grid::from(SAMPLE_DATA);
        assert_eq!(grid.step_until_all_flashing(), 195);
    }
}
