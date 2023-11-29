//! Common

use aoc_sx::itertools::Itertools;
use aoc_sx::petgraph::{
    algo,
    graph::{DiGraph, NodeIndex},
};

pub struct Cave {
    array: Vec<Vec<u64>>,
    graph: DiGraph<u64, u64>,
}

impl Cave {
    pub fn get_lower_risk_path_sum(&self) -> u64 {
        let (cost, _) = self.get_min_path();
        cost
    }

    pub fn get_min_path(&self) -> (u64, Vec<NodeIndex>) {
        let target = NodeIndex::new(self.graph.node_count() - 1);
        algo::astar(
            &self.graph,
            0.into(),
            |x| x == target,
            |e| *e.weight(),
            |_| 0,
        )
        .unwrap()
    }

    fn round_to_1(v: u64) -> u64 {
        let mut v = v;
        while v > 9 {
            v -= 9
        }
        v
    }

    pub fn create_full_map(&self) -> Self {
        let multiplicator = 5;
        let initial_width = self.array[0].len();
        let initial_height = self.array.len();
        let width = initial_width * multiplicator;
        let height = initial_height * multiplicator;

        let compute_offset = |x: usize, y: usize| -> u64 {
            let offset_x = (x as f32 / initial_width as f32) as u64;
            let offset_y = (y as f32 / initial_height as f32) as u64;
            offset_x + offset_y
        };

        let mut node_weights = vec![];
        let mut edge_weights = vec![];
        for y in 0..height {
            for x in 0..width {
                let idx = (x + y * width) as u32;
                let v = Self::round_to_1(
                    self.array[y % initial_height][x % initial_width] + compute_offset(x, y),
                );
                node_weights.push((idx, v));

                if x != width - 1 {
                    let other_v = Self::round_to_1(
                        self.array[y % initial_height][(x + 1) % initial_width]
                            + compute_offset(x + 1, y),
                    );
                    edge_weights.push((idx, idx + 1, other_v));
                    edge_weights.push((idx + 1, idx, v));
                }

                if y != height - 1 {
                    let other_v = Self::round_to_1(
                        self.array[(y + 1) % initial_height][x % initial_width]
                            + compute_offset(x, y + 1),
                    );
                    edge_weights.push((idx, idx + width as u32, other_v));
                    edge_weights.push((idx + width as u32, idx, v));
                }
            }
        }

        let mut graph = DiGraph::new();
        for (_, w) in node_weights {
            graph.add_node(w);
        }

        for (e1, e2, w) in edge_weights {
            graph.update_edge(e1.into(), e2.into(), w);
        }

        Self {
            array: self.array.clone(),
            graph,
        }
    }
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        let array = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|x| x.to_digit(10).unwrap() as u64)
                    .collect_vec()
            })
            .collect_vec();
        let width = array[0].len();
        let height = array.len();

        let mut edges = vec![];
        for (y, l) in array.iter().enumerate() {
            for (x, &v) in l.iter().enumerate() {
                let idx = (x + y * width) as u32;

                if x != width - 1 {
                    let other_v = l[x + 1];
                    edges.push((idx, idx + 1, other_v));
                    edges.push((idx + 1, idx, v));
                }

                if y != height - 1 {
                    let other_v = array[y + 1][x];
                    edges.push((idx, idx + width as u32, other_v));
                    edges.push((idx + width as u32, idx, v));
                }
            }
        }

        Self {
            array,
            graph: DiGraph::from_edges(edges),
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::Cave;

    const SMALL_SAMPLE_DATA: &str = indoc! {"
        1163
        1381
        2136
        3694"
    };

    const SAMPLE_DATA: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581"
    };

    const SAMPLE_DATA_2: &str = indoc! {"
        1911191111
        1119111991
        9999999111
        9999911199
        9999119999
        9999199999
        9111199999
        9199999111
        9111911191
        9991119991"
    };

    #[test]
    fn test_small_sample() {
        let cave = Cave::from(SMALL_SAMPLE_DATA);
        assert_eq!(cave.get_lower_risk_path_sum(), 17);
    }

    #[test]
    fn test_sample() {
        let cave = Cave::from(SAMPLE_DATA);
        assert_eq!(cave.get_lower_risk_path_sum(), 40);
    }

    #[test]
    fn test_sample_2() {
        let cave = Cave::from(SAMPLE_DATA_2);
        assert_eq!(cave.get_lower_risk_path_sum(), 40);
    }

    #[test]
    fn test_sample_x5() {
        let cave = Cave::from(SAMPLE_DATA);
        let cave = cave.create_full_map();
        assert_eq!(cave.get_lower_risk_path_sum(), 315);
    }
}
