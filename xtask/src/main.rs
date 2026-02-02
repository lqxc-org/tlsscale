use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Development automation scripts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run code formatting
    Fmt,
    /// Run clippy
    Lint,
    /// Run tests
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Fmt => {
            println!("Running cargo fmt...");
            std::process::Command::new("cargo")
                .arg("fmt")
                .status()?;
        }
        Commands::Lint => {
            println!("Running cargo clippy...");
            std::process::Command::new("cargo")
                .arg("clippy")
                .args(&["--workspace", "--all-targets", "--all-features"])
                .status()?;
        }
        Commands::Test => {
            println!("Running cargo test...");
            std::process::Command::new("cargo")
                .arg("test")
                .arg("--workspace")
                .status()?;
        }
    }

    Ok(())
}