//! Common

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use aoc_sx::itertools::Itertools;

pub fn save_graph_to_disk(graph: &Graph, output: &Path) {
    let mut dot_output = String::new();
    dot_output.push_str("graph G {\n");

    for (&node_id, node_links) in &graph.links {
        let node_name = &graph.nodes[node_id].name;
        for &target_id in node_links {
            let target_name = &graph.nodes[target_id].name;
            dot_output.push_str(&format!("  {} -- {}\n", node_name, target_name));
        }
    }

    dot_output.push('}');

    std::fs::write(output, dot_output).expect("Could not write DOT to disk.");
}

#[derive(Debug)]
pub struct Node {
    name: String,
    is_small_cave: bool,
}

impl Node {
    pub fn new(name: String) -> Self {
        let small = name.to_lowercase() == name;
        Self {
            name,
            is_small_cave: small,
        }
    }

    pub fn is_small_cave(&self) -> bool {
        self.is_small_cave
    }

    pub fn is_end(&self) -> bool {
        self.name == "end"
    }

    pub fn is_start(&self) -> bool {
        self.name == "start"
    }
}

#[derive(Debug)]
pub struct Graph {
    index: HashMap<String, usize>,
    nodes: Vec<Node>,
    links: HashMap<usize, HashSet<usize>>,
}

#[derive(Debug)]
pub struct Tree<T> {
    node: T,
    children: Vec<Tree<T>>,
}

impl<T: Clone + std::fmt::Debug> Tree<T> {
    pub fn new(elem: T) -> Self {
        Self {
            node: elem,
            children: Vec::new(),
        }
    }

    pub fn list_paths(&self) -> Vec<Vec<T>> {
        Self::list_paths_rec(self, vec![])
    }

    pub fn list_paths_rec(tree: &Self, mut path: Vec<T>) -> Vec<Vec<T>> {
        let mut out = vec![];
        path.push(tree.node.clone());

        if tree.children.is_empty() {
            out.push(path.clone());
        }

        for child in &tree.children {
            let new_path = path.clone();
            for p in Self::list_paths_rec(child, new_path) {
                out.push(p)
            }
        }

        out
    }
}

impl Graph {
    pub fn find_paths(&self) -> Vec<String> {
        let start_id = self.index["start"];
        let mut seen = HashSet::new();
        seen.insert(start_id);

        let tree = self.iterate_paths(start_id, seen);
        let paths = tree.list_paths();

        paths
            .iter()
            .filter(|l| self.nodes[l[l.len() - 1]].is_end())
            .map(|l| l.iter().map(|&x| self.nodes[x].name.clone()).join(","))
            .collect()
    }

    pub fn find_paths2(&self) -> Vec<String> {
        let mut final_paths: Vec<Vec<usize>> = vec![];

        for small_cave_id in self.list_small_caves() {
            let start_id = self.index["start"];
            let mut seen = HashMap::new();
            seen.insert(start_id, 1);

            let tree = self.iterate_paths2(start_id, seen, small_cave_id);
            let mut paths = tree.list_paths();
            final_paths.append(&mut paths);
        }

        final_paths
            .iter()
            .unique()
            .filter(|l| self.nodes[l[l.len() - 1]].is_end())
            .map(|l| l.iter().map(|&x| self.nodes[x].name.clone()).join(","))
            .collect()
    }

    fn list_small_caves(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .filter(|&n| n.is_small_cave() && !n.is_end() && !n.is_start())
            .map(|x| self.index[&x.name])
            .collect()
    }

    fn get_links_for_node_id(&self, node_id: usize) -> &HashSet<usize> {
        &self.links[&node_id]
    }

    fn iterate_paths(&self, node_id: usize, seen: HashSet<usize>) -> Tree<usize> {
        let current_node = &self.nodes[node_id];
        if current_node.is_end() {
            return Tree::new(node_id);
        }

        let mut out = Tree::new(node_id);
        for &link_node_id in self.get_links_for_node_id(node_id) {
            let mut seen = seen.clone();
            let node = &self.nodes[link_node_id];
            if node.is_small_cave() {
                if !seen.contains(&link_node_id) {
                    seen.insert(link_node_id);
                } else {
                    continue;
                }
            }

            out.children.push(self.iterate_paths(link_node_id, seen));
        }

        out
    }

    fn iterate_paths2(
        &self,
        node_id: usize,
        seen: HashMap<usize, usize>,
        twice_id: usize,
    ) -> Tree<usize> {
        let current_node = &self.nodes[node_id];
        if current_node.is_end() {
            return Tree::new(node_id);
        }

        let mut out = Tree::new(node_id);
        for &link_node_id in self.get_links_for_node_id(node_id) {
            let mut seen = seen.clone();
            let node = &self.nodes[link_node_id];
            if node.is_small_cave() {
                if seen.contains_key(&link_node_id) {
                    if link_node_id == twice_id {
                        if seen[&link_node_id] > 1 {
                            continue;
                        } else {
                            seen.insert(link_node_id, seen[&link_node_id] + 1);
                        }
                    } else {
                        continue;
                    }
                } else {
                    seen.insert(link_node_id, 1);
                }
            }

            out.children
                .push(self.iterate_paths2(link_node_id, seen, twice_id));
        }

        out
    }
}

impl From<&[&str]> for Graph {
    fn from(s: &[&str]) -> Self {
        let mut index = HashMap::new();
        let mut nodes = vec![];
        let mut links = HashMap::new();

        for &line in s {
            let mut split = line.split('-');
            let a: String = split.next().unwrap().into();
            let b: String = split.next().unwrap().into();

            let a_pos = if !index.contains_key(&a) {
                let pos = nodes.len();
                nodes.push(Node::new(a.clone()));
                index.insert(a.clone(), pos);
                pos
            } else {
                index[&a]
            };

            let b_pos = if !index.contains_key(&b) {
                let pos = nodes.len();
                nodes.push(Node::new(b.clone()));
                index.insert(b.clone(), pos);
                pos
            } else {
                index[&b]
            };

            links
                .entry(a_pos)
                .or_insert_with(HashSet::new)
                .insert(b_pos);
            links
                .entry(b_pos)
                .or_insert_with(HashSet::new)
                .insert(a_pos);
        }

        Self {
            index,
            nodes,
            links,
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::algo::fs::get_debug_path;

    use super::{save_graph_to_disk, Graph};

    const SAMPLE_DATA: &[&str] = &["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
    const LARGER_SAMPLE_DATA: &[&str] = &[
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];
    const EVEN_LARGER_SAMPLE_DATA: &[&str] = &[
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn test_sample() {
        let graph = Graph::from(SAMPLE_DATA);
        let path = get_debug_path().join("aoc2021-day12-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths().len(), 10);
    }

    #[test]
    fn test_sample_2() {
        let graph = Graph::from(SAMPLE_DATA);
        let path = get_debug_path().join("aoc2021-day12-sample-2.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths2().len(), 36);
    }

    #[test]
    fn test_larger_sample() {
        let graph = Graph::from(LARGER_SAMPLE_DATA);
        let path = get_debug_path().join("aoc2021-day12-larger-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths().len(), 19);
    }

    #[test]
    fn test_larger_sample_2() {
        let graph = Graph::from(LARGER_SAMPLE_DATA);
        let path = get_debug_path().join("aoc2021-day12-larger-sample-2.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths2().len(), 103);
    }

    #[test]
    fn test_even_larger_sample() {
        let graph = Graph::from(EVEN_LARGER_SAMPLE_DATA);
        let path = get_debug_path().join("aoc2021-day12-even-larger-sample.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths().len(), 226);
    }

    #[test]
    fn test_even_larger_sample_2() {
        let graph = Graph::from(EVEN_LARGER_SAMPLE_DATA);
        let path = get_debug_path().join("aoc2021-day12-even-larger-sample2.dot");
        save_graph_to_disk(&graph, &path);

        assert_eq!(graph.find_paths2().len(), 3509);
    }
}
