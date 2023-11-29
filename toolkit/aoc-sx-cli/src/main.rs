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
    }

    Ok(())
}
