pub mod config;
pub mod logging;

use dotenvy::dotenv;
use std::path::PathBuf;

use aoc_sx_codegen::{ModuleGenerator, ModuleParameters};
use aoc_sx_core::exercise::{ExerciseDay, ExercisePart, ExerciseYear};
use color_eyre::Result;

use aoc_sx_webclient::Client;
use clap::Parser;
use config::Config;

/// AoC Sx Toolkit
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Command to execute
    #[clap(subcommand)]
    command: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    /// Generate day module
    Codegen(GenerateDayModule),
    /// Fetch missing inputs
    FetchMissingInputs(FetchMissingInputs),
    /// Send answer
    SendAnswer(SendAnswer),
}

#[derive(Debug, Parser)]
struct GenerateDayModule {
    /// Session token.
    #[arg(long)]
    pub session_token: Option<String>,
    /// Year.
    #[arg(short, long)]
    pub year: ExerciseYear,
    /// Day
    #[arg(short, long)]
    pub day: ExerciseDay,
}

#[derive(Debug, Parser)]
struct SendAnswer {
    /// Session token.
    #[arg(long)]
    pub session_token: Option<String>,
    /// Year.
    #[arg(short, long)]
    pub year: ExerciseYear,
    /// Day.
    #[arg(short, long)]
    pub day: ExerciseDay,
    /// Part.
    #[arg(short, long)]
    pub part: ExercisePart,
    /// Answer
    pub answer: String,
}

#[derive(Debug, Parser)]
struct FetchMissingInputs {
    /// Session token.
    #[arg(long)]
    pub session_token: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    if dotenv().is_err() {
        println!("Env file not found, skipping.")
    }

    crate::logging::Logging::setup()?;

    let args = Args::parse();
    let config = Config::from_env()?;

    match args.command {
        Subcommand::Codegen(GenerateDayModule {
            session_token,
            year,
            day,
        }) => {
            let token = session_token
                .or(config.session_token)
                .expect("Missing session token.");

            let path = PathBuf::from(".")
                .join("editions")
                .join(format!("aoc-{year}"))
                .join("src");
            let generator = ModuleGenerator::new(token);
            generator.generate_module(path, ModuleParameters { year, day })?;
        }

        Subcommand::SendAnswer(SendAnswer {
            session_token,
            year,
            day,
            part,
            answer,
        }) => {
            let token = session_token
                .or(config.session_token)
                .expect("Missing session token");
            let client = Client::new(token);
            let response = client.send_answer(&answer, year, day, part)?;
            println!("{:?}", response);
        }

        Subcommand::FetchMissingInputs(FetchMissingInputs { session_token }) => {
            let token = session_token
                .or(config.session_token)
                .expect("Missing session token");
            let client = Client::new(token);

            // Scan for missing inputs in editions
            let editions_path = PathBuf::from(".").join("editions");

            for year_folder in std::fs::read_dir(editions_path)? {
                let year_folder = year_folder?;
                if year_folder.path().is_file() {
                    continue;
                }

                let folder_name = year_folder.file_name().to_string_lossy().to_string();
                let folder_name_split = folder_name.split('-').collect::<Vec<_>>();
                if folder_name_split.len() != 2 {
                    panic!("Edition folder should match 'aoc-[year]'.");
                }

                let year = ExerciseYear::try_from(folder_name_split[1].parse::<u16>()?)?;

                for day_folder in std::fs::read_dir(year_folder.path().join("src"))? {
                    let day_folder = day_folder?;
                    if day_folder.path().is_file() {
                        continue;
                    }

                    let folder_name = day_folder.file_name().to_string_lossy().to_string();
                    let (folder_name_title, folder_name_value) = folder_name.split_at(3);
                    if folder_name_title != "day" {
                        panic!("Day folder should match 'day[day]' (input: {folder_name_title})");
                    }

                    let day = ExerciseDay::try_from(folder_name_value.parse::<u8>()?)?;

                    // Check for input.txt
                    let input_txt_path = day_folder.path().join("input.txt");
                    if !input_txt_path.exists() {
                        let puzzle_input = client.fetch_input_page(year, day)?;
                        println!("Creating {input_txt_path:?} ...");
                        std::fs::write(&input_txt_path, puzzle_input.as_str())?;
                    }
                }
            }
        }
    }

    Ok(())
}
