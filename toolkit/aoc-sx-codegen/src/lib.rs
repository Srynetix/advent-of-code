pub mod parser;

use std::path::Path;

use aoc_sx_core::exercise::{ExerciseDay, ExerciseYear};
use aoc_sx_webclient::Client;
use itertools::Itertools;
use parser::MarkdownContent;

use crate::parser::DayPageParser;

#[derive(Debug)]
pub struct ModuleGenerator {
    client: Client,
}

#[derive(Debug)]
pub struct ModuleParameters {
    pub year: ExerciseYear,
    pub day: ExerciseDay,
}

impl ModuleGenerator {
    pub fn new(session_token: String) -> Self {
        Self {
            client: Client::new(session_token),
        }
    }

    /// Generate a Rust module.
    pub fn generate_module<P: AsRef<Path>>(
        &self,
        folder: P,
        parameters: ModuleParameters,
    ) -> std::io::Result<()> {
        let exercise_page = self
            .client
            .fetch_exercise_page(parameters.year, parameters.day)
            .unwrap();

        let markdown = DayPageParser::new(exercise_page.page_url, exercise_page.input_url)
            .parse(exercise_page.page_content.as_str());

        let path = folder
            .as_ref()
            .join(format!("day{:02}", parameters.day.as_u8()));
        if !path.exists() {
            // Create top directory
            println!("Creating {path:?} ...");
            std::fs::create_dir_all(&path)?;
        }

        // Then, create mod.rs
        let mod_rs = path.join("mod.rs");
        println!("Creating {mod_rs:?} ...");
        std::fs::write(&mod_rs, self.markdown_to_mod_rs(&markdown))?;

        // Then, create part1.rs
        let part1_rs = path.join("part1.rs");
        if !part1_rs.exists() {
            println!("Creating {part1_rs:?} ...");
            std::fs::write(&part1_rs, self.scaffold_code_part("Part 1"))?;
        }

        // Then, create part2.rs
        let part2_rs = path.join("part2.rs");
        if !part2_rs.exists() {
            println!("Creating {part2_rs:?} ...");
            std::fs::write(&part2_rs, self.scaffold_code_part("Part 2"))?;
        }

        // Then, create common.rs
        let common_rs = path.join("common.rs");
        if !common_rs.exists() {
            println!("Creating {common_rs:?} ...");
            std::fs::write(&common_rs, "//! Common")?;
        }

        // Finally, create input.txt
        let input_txt = path.join("input.txt");
        if !input_txt.exists() {
            println!("Creating {input_txt:?} ...");
            std::fs::write(&input_txt, exercise_page.puzzle_input.as_str())?;
        }

        // Generate root module
        let root_module = path.parent().unwrap();
        let lib_rs = root_module.join("lib.rs");
        println!("Creating {lib_rs:?} ...");
        std::fs::write(&lib_rs, self.generate_root_module(root_module))?;

        Ok(())
    }

    fn markdown_to_mod_rs(&self, md: &MarkdownContent) -> String {
        let mut output = String::new();
        output.push_str(&self.markdown_to_rust_comment(md));

        // Module declarations
        output.push('\n');
        output.push_str("pub mod common;\n");
        output.push_str("pub mod part1;\n");
        output.push_str("pub mod part2;\n");
        output.push('\n');
        output.push_str("const INPUT: &str = include_str!(\"./input.txt\");\n");

        output
    }

    fn markdown_to_rust_comment(&self, md: &MarkdownContent) -> String {
        let mut output = String::new();

        for line in md.as_str().lines() {
            if line.is_empty() {
                output.push_str("//!\n");
            } else {
                output.push_str(&format!("//! {line}\n"));
            }
        }

        output
    }

    fn scaffold_code_part(&self, title: &str) -> String {
        let mut output = String::new();

        output.push_str(&format!("//! {title}\n\n"));
        output.push_str(indoc::indoc! {r###"
            use super::INPUT;

            pub fn run() -> usize {
                0
            }

            #[cfg(test)]
            mod tests {
                #[test]
                fn run() {
                    assert_eq!(super::run(), 0)
                }
            }
        "###});

        output
    }

    fn generate_root_module(&self, path: &Path) -> String {
        let mut output = String::new();

        let directory = std::fs::read_dir(path).unwrap();
        for file in directory.map(|e| e.unwrap().file_name()).sorted() {
            let filename = file.into_string().unwrap();
            if !filename.ends_with(".rs") {
                output.push_str(&format!("pub mod {filename};\n"));
            }
        }

        output
    }
}
