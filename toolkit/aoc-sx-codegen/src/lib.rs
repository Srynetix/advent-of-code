pub mod parser;

use std::path::Path;

use aoc_sx_core::exercise::{ExerciseDay, ExerciseYear};
use aoc_sx_webclient::Client;
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
    ///
    /// Should look like this:
    /// - ./path/to/module/mod.rs
    /// - ./path/to/module/part1.rs
    /// - ./path/to/module/part2.rs
    /// - ./path/to/module/common.rs
    /// - ./path/to/module/input.txt
    ///
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
        println!("Creating {input_txt:?} ...");
        std::fs::write(&input_txt, exercise_page.puzzle_input.as_str())?;

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
            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn run() {
                    todo!()
                }
            }
        "###});

        output
    }
}
