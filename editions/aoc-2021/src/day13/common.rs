//! Common

use std::{cmp::Ordering, collections::LinkedList, fmt::Display, path::Path};

use aoc_sx::image::ImageBuffer;

fn state_to_color(state: bool) -> &'static [u8; 3] {
    // Palette from https://coolors.co/d5bff2-d3bae3-d1b6d6-b197bd-706791-4e507a-364066-1d3057-10274f-031e47
    match state {
        false => &[0, 0, 0],
        true => &[255, 255, 255],
    }
}

pub fn save_paper_to_disk(map: &TransparentPaper, output: &Path) {
    let mut buffer = ImageBuffer::new(map.width as u32, map.height as u32);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let pos = map.xy_to_pos(x as usize, y as usize);
        let state = map.data[pos];
        *pixel = aoc_sx::image::Rgb(*state_to_color(state));
    }

    buffer.save(output).expect("Could not generate paper.");
}

#[derive(Clone, Copy)]
pub enum Coordinate {
    X,
    Y,
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "y" => Self::Y,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct FoldingRule {
    coordinate: Coordinate,
    value: u32,
}

impl From<&str> for FoldingRule {
    fn from(s: &str) -> Self {
        let value: String = s.chars().skip("fold along ".len()).collect();
        let mut split = value.split('=');
        let coordinate = Coordinate::from(split.next().unwrap());
        let value: u32 = split.next().and_then(|x| x.parse().ok()).unwrap();

        Self { coordinate, value }
    }
}

pub struct TransparentPaper {
    width: usize,
    height: usize,
    data: Vec<bool>,
    rules: LinkedList<FoldingRule>,
}

impl TransparentPaper {
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

    pub fn count_dots(&self) -> usize {
        self.data.iter().filter(|&&x| x).count()
    }

    pub fn fold_next_rule(&self) -> Option<Self> {
        let mut rules = self.rules.clone();
        if let Some(rule) = rules.pop_front() {
            match rule.coordinate {
                Coordinate::X => {
                    let width = rule.value as usize;
                    let height = self.height;
                    let mut data = vec![false; width * height];

                    for (idx, &v) in self.data.iter().enumerate() {
                        let (cur_x, cur_y) = self.pos_to_xy(idx);

                        match cur_x.cmp(&(rule.value as usize)) {
                            Ordering::Greater => {
                                let diff_x = cur_x - rule.value as usize;
                                let inv_x = rule.value as usize - diff_x;
                                let inv_c = inv_x + cur_y * width;
                                if v {
                                    data[inv_c] = v;
                                }
                            }
                            Ordering::Less => {
                                let idx = cur_x + cur_y * width;
                                if v {
                                    data[idx] = v;
                                }
                            }
                            _ => (),
                        }
                    }

                    Some(Self {
                        width,
                        height,
                        rules,
                        data,
                    })
                }
                Coordinate::Y => {
                    let width = self.width;
                    let height = rule.value as usize;
                    let middle_point_start = rule.value as usize * self.width;
                    let middle_point_end = middle_point_start + self.width - 1;
                    let mut data = vec![false; width * height];

                    for (idx, &x) in self.data[..middle_point_start].iter().enumerate() {
                        data[idx] = x;
                    }

                    for (idx, &x) in self.data[middle_point_end..].iter().enumerate() {
                        if x {
                            let (cur_x, cur_y) = self.pos_to_xy(middle_point_end + idx);
                            let diff_y = cur_y - rule.value as usize;
                            let inv_y = rule.value as usize - diff_y;
                            let inv_c = self.xy_to_pos(cur_x, inv_y);
                            data[inv_c] = x;
                        }
                    }

                    Some(Self {
                        width,
                        height,
                        rules,
                        data,
                    })
                }
            }
        } else {
            None
        }
    }
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, &v) in self.data.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f)?;
            }

            if v {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }

        Ok(())
    }
}

impl From<&str> for TransparentPaper {
    fn from(s: &str) -> Self {
        let mut split = s.split("\n\n");
        let coords = split.next().unwrap();
        let rules = split.next().unwrap();

        let coord_lines: Vec<_> = coords.lines().collect();
        let positions: Vec<(u32, u32)> = coord_lines
            .iter()
            .map(|x| {
                let mut s = x.split(',');
                (
                    s.next().and_then(|x| x.parse().ok()).unwrap(),
                    s.next().and_then(|x| x.parse().ok()).unwrap(),
                )
            })
            .collect();

        let width = positions.iter().map(|(x, _)| x).max().copied().unwrap() as usize + 1;
        let height = positions.iter().map(|(_, y)| y).max().copied().unwrap() as usize + 1;
        let mut data = vec![false; width * height];

        for (x, y) in positions {
            let coord_u = (x + y * width as u32) as usize;
            data[coord_u] = true;
        }

        let rules: LinkedList<_> = rules.lines().map(FoldingRule::from).collect();

        Self {
            width,
            height,
            data,
            rules,
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::TransparentPaper;

    const SAMPLE_INPUT: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn test_sample() {
        let paper = TransparentPaper::from(SAMPLE_INPUT);
        assert_eq!(
            paper.to_string(),
            indoc! {"
            ...#..#..#.
            ....#......
            ...........
            #..........
            ...#....#.#
            ...........
            ...........
            ...........
            ...........
            ...........
            .#....#.##.
            ....#......
            ......#...#
            #..........
            #.#........"
            }
        );

        let folded_once = paper.fold_next_rule().unwrap();
        assert_eq!(
            folded_once.to_string(),
            indoc! {"
            #.##..#..#.
            #...#......
            ......#...#
            #...#......
            .#.#..#.###
            ...........
            ..........."
            }
        );
        assert_eq!(folded_once.count_dots(), 17);

        let folded_twice = folded_once.fold_next_rule().unwrap();
        assert_eq!(
            folded_twice.to_string(),
            indoc! {"
                #####
                #...#
                #...#
                #...#
                #####
                .....
                ....."
            }
        );
    }
}
