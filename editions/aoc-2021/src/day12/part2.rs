//! Part 2

use aoc_sx::algo::{fs::get_debug_path, parse::parse_str_lines};

use super::{
    common::{save_graph_to_disk, Graph},
    INPUT,
};

pub fn run() -> usize {
    let graph = Graph::from(&parse_str_lines(INPUT)[..]);
    let path = get_debug_path().join("aoc2021-day12.dot");
    save_graph_to_disk(&graph, &path);

    graph.find_paths2().len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 128_506)
    }
}
