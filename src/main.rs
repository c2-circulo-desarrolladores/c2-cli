use c2_cli::core::Deployable;
use c2_cli::deployables::executables::{Format, Release};
use c2_cli::deployables::importables::{Api, Logger, Timer};
use c2_cli::deployables::init::Init;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "c2-cli",
    about = "CLI tool to initialize open source projects and importing reusable modules (Python-focused)"
)]
#[command(version)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes Python project with .gitignore, justfile and .github/
    Init {
        /// GitHub owner/org used to fill '<OWNER>' in cliff.toml (optional)
        #[arg(long)]
        owner: Option<String>,
    },

    /// Copies a reusable module into the current project (timer, logger, api...)
    Import { target: ImportTarget },

    /// Calls ruff, isort and autoflake to format Python code
    #[command(name = "format")]
    Format,

    /// Full release: bump version, generate changelog, commit, tag and push
    Release,
}

#[derive(ValueEnum, Clone)]
enum ImportTarget {
    Timer,
    Logger,
    Api,
}

fn main() -> std::io::Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Init { owner } => {
            Init { owner }.deploy()?;
        }

        Commands::Import { target } => match target {
            ImportTarget::Timer => Timer {}.deploy()?,
            ImportTarget::Logger => Logger {}.deploy()?,
            ImportTarget::Api => Api {}.deploy()?,
        },

        Commands::Format => {
            Format {}.deploy()?;
        }

        Commands::Release => {
            Release {}.deploy()?;
        }
    }

    Ok(())
}
