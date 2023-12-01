//! Common

use aoc_sx::itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum ShellCommand {
    Ls,
    Cd(String),
}

impl ShellCommand {
    pub fn parse_line(line: &str) -> Self {
        let spl = line.split_whitespace().collect_vec();
        assert!(spl.len() >= 2);

        let cmd = spl[1];
        if cmd == "ls" {
            Self::Ls
        } else {
            assert!(spl.len() == 3);
            Self::Cd(spl[2].to_string())
        }
    }
}

#[derive(Debug)]
pub struct Filesystem {
    root: FilesystemNode,
}

impl Default for Filesystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Filesystem {
    pub const MAX_SIZE: usize = 70_000_000;
    pub const REQUIRED_SPACE: usize = 30_000_000;

    pub fn new() -> Self {
        Self {
            root: FilesystemNode::Folder(FilesystemFolder::new("/")),
        }
    }

    pub fn get_remaining_space(&self) -> usize {
        Self::MAX_SIZE - self.root.total_size()
    }

    pub fn add_node(&mut self, path: &ShellDirectory, node: FilesystemNode) {
        let target = self.get_node_at_path_mut(path);
        if let FilesystemNode::Folder(f) = target {
            f.add_node(node);
        }
    }

    pub fn dump_tree_to_string(&self) -> String {
        let mut output = String::new();
        self.root.dump_into_string(0, &mut output);
        output
    }

    fn get_node_at_path(&self, path: &ShellDirectory) -> &FilesystemNode {
        let mut cursor = &self.root;

        for component in path.components.iter().skip(1) {
            if let FilesystemNode::Folder(node) = cursor {
                cursor = node.get_child_by_name(component);
            } else {
                panic!("Cannot traverse a file!");
            }
        }

        cursor
    }

    fn get_node_at_path_mut(&mut self, path: &ShellDirectory) -> &mut FilesystemNode {
        let mut cursor = &mut self.root;

        for component in path.components.iter().skip(1) {
            if let FilesystemNode::Folder(node) = cursor {
                cursor = node.get_child_by_name_mut(component);
            } else {
                panic!("Cannot traverse a file!");
            }
        }

        cursor
    }

    pub fn get_path_total_size(&self, path: &ShellDirectory) -> usize {
        let node = self.get_node_at_path(path);
        node.total_size()
    }
}

#[derive(Debug)]
pub enum FilesystemNode {
    Folder(FilesystemFolder),
    File(FilesystemFile),
}

impl FilesystemNode {
    pub fn from_input(input: &str) -> Self {
        let spl = input.split_whitespace().collect_vec();
        assert_eq!(spl.len(), 2);

        if spl[0] == "dir" {
            Self::Folder(FilesystemFolder::new(spl[1]))
        } else {
            Self::File(FilesystemFile::new(
                spl[1],
                spl[0].parse().expect("should be a number"),
            ))
        }
    }

    pub fn dump_into_string(&self, prefix: usize, output: &mut String) {
        for _ in 0..prefix {
            output.push(' ');
        }

        output.push_str("- ");
        match self {
            FilesystemNode::File(f) => {
                output.push_str(&format!("{} (file, size={})\n", f.name, f.size));
            }
            FilesystemNode::Folder(f) => {
                output.push_str(&format!("{} (dir)\n", f.name));
                for child in &f.files {
                    child.dump_into_string(prefix + 2, output);
                }
            }
        }
    }

    pub fn total_size(&self) -> usize {
        match self {
            FilesystemNode::File(f) => f.size,
            FilesystemNode::Folder(f) => f.total_size(),
        }
    }

    pub fn iterate_directories(&self) -> Vec<&FilesystemFolder> {
        let mut directories = vec![];

        if let FilesystemNode::Folder(f) = &self {
            for child in &f.files {
                if let FilesystemNode::Folder(f) = child {
                    directories.push(f);
                    directories.append(&mut child.iterate_directories());
                }
            }
        }

        directories
    }

    pub fn get_directories_with_size_less_than(&self, count: usize) -> Vec<&FilesystemFolder> {
        let mut directories = vec![];

        if let FilesystemNode::Folder(f) = &self {
            for child in &f.files {
                if let FilesystemNode::Folder(f) = child {
                    if f.total_size() <= count {
                        directories.push(f);
                    }
                    directories.append(&mut child.get_directories_with_size_less_than(count));
                }
            }
        }

        directories
    }
}

#[derive(Debug)]
pub struct ShellDirectory {
    components: Vec<String>,
}

impl ShellDirectory {
    pub fn from_input(input: &str) -> Self {
        Self {
            components: input
                .trim_end_matches('/')
                .split('/')
                .map(|x| x.to_string())
                .collect(),
        }
    }

    pub fn cd(&mut self, path: &str) {
        if path == "/" {
            self.components = vec!["".to_string()];
        } else if path == ".." {
            self.components.pop();
        } else {
            self.components.push(path.to_string());
        }
    }

    pub fn is_root(&self) -> bool {
        self.components == [""]
    }
}

#[derive(Debug)]
pub struct FilesystemFile {
    name: String,
    size: usize,
}

impl FilesystemFile {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }
}

#[derive(Debug)]
pub struct FilesystemFolder {
    name: String,
    files: Vec<FilesystemNode>,
}

impl FilesystemFolder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            files: vec![],
        }
    }

    pub fn add_node(&mut self, node: FilesystemNode) {
        self.files.push(node);
    }

    pub fn get_child_by_name<'a>(&'a self, name: &str) -> &'a FilesystemNode {
        for child in &self.files {
            match child {
                FilesystemNode::File(f) if f.name == name => return child,
                FilesystemNode::Folder(f) if f.name == name => return child,
                _ => (),
            }
        }

        panic!("Child not found!")
    }

    pub fn get_child_by_name_mut<'a>(&'a mut self, name: &str) -> &'a mut FilesystemNode {
        for child in &mut self.files {
            match child {
                FilesystemNode::File(f) if f.name == name => return child,
                FilesystemNode::Folder(f) if f.name == name => return child,
                _ => (),
            }
        }

        panic!("Child not found!")
    }

    pub fn total_size(&self) -> usize {
        self.files.iter().map(|f| f.total_size()).sum()
    }
}

#[derive(Debug)]
pub struct ShellSession {
    fs: Filesystem,
    current_directory: ShellDirectory,
}

impl Default for ShellSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellSession {
    pub fn new() -> Self {
        Self {
            fs: Filesystem::new(),
            current_directory: ShellDirectory::from_input("/"),
        }
    }

    pub fn from_input(input: &str) -> Self {
        let mut instance = Self::new();
        instance.parse_input(input);
        instance
    }

    pub fn parse_input(&mut self, input: &str) {
        for line in input.trim().lines() {
            self.parse_line(line.trim());
        }
    }

    pub fn dump_tree_to_string(&self) -> String {
        self.fs.dump_tree_to_string()
    }

    pub fn sum_directories_total_size_less_than(&self, count: usize) -> usize {
        self.fs
            .root
            .get_directories_with_size_less_than(count)
            .into_iter()
            .map(|x| x.total_size())
            .sum()
    }

    pub fn find_smallest_directory_to_free_space(&self) -> &FilesystemFolder {
        let space_to_delete = Filesystem::REQUIRED_SPACE - self.fs.get_remaining_space();
        self.fs
            .root
            .iterate_directories()
            .into_iter()
            .filter(|x| x.total_size() >= space_to_delete)
            .sorted_by_key(|k| k.total_size())
            .next()
            .unwrap()
    }

    fn parse_line(&mut self, line: &str) {
        if line.starts_with('$') {
            let command = ShellCommand::parse_line(line);
            match command {
                ShellCommand::Ls => {}
                ShellCommand::Cd(directory) => {
                    self.current_directory.cd(&directory);
                }
            }
        } else {
            // Dir output
            let node = FilesystemNode::from_input(line);
            self.fs.add_node(&self.current_directory, node);
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_sx::{indoc::indoc, itertools::Itertools};

    use crate::day07::common::ShellDirectory;

    use super::{Filesystem, ShellSession};

    const SAMPLE: &str = indoc! {r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "#};

    const VISUAL_SAMPLE: &str = indoc! {r#"
    - / (dir)
      - a (dir)
        - e (dir)
          - i (file, size=584)
        - f (file, size=29116)
        - g (file, size=2557)
        - h.lst (file, size=62596)
      - b.txt (file, size=14848514)
      - c.dat (file, size=8504156)
      - d (dir)
        - j (file, size=4060174)
        - d.log (file, size=8033020)
        - d.ext (file, size=5626152)
        - k (file, size=7214296)
    "#};

    #[test]
    fn parse() {
        let sample = ShellSession::from_input(SAMPLE);
        assert_eq!(sample.dump_tree_to_string(), VISUAL_SAMPLE);

        assert_eq!(
            sample
                .fs
                .get_path_total_size(&ShellDirectory::from_input("/a/e")),
            584
        );
        assert_eq!(
            sample
                .fs
                .get_path_total_size(&ShellDirectory::from_input("/a")),
            94_853
        );
        assert_eq!(
            sample
                .fs
                .get_path_total_size(&ShellDirectory::from_input("/d")),
            24_933_642
        );
        assert_eq!(
            sample
                .fs
                .get_path_total_size(&ShellDirectory::from_input("/")),
            48_381_165
        );
    }

    #[test]
    fn at_most_100000() {
        let sample = ShellSession::from_input(SAMPLE);
        assert_eq!(sample.sum_directories_total_size_less_than(100_000), 95_437);
    }

    #[test]
    fn to_free() {
        let sample = ShellSession::from_input(SAMPLE);
        assert_eq!(sample.find_smallest_directory_to_free_space().name, "d");
    }
}
