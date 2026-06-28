use c2_cli::core::deployable::Deployable;
use c2_cli::core::gen_deployable::{Api, FormatPy, Init, Logger, Timer};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "c2-cli",
    about = "CLI tool to initialize open source projects and importing reusable modules (Python-focused)"
)]
struct CLI {
    /// Hí
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes Python project with .gitignore, justfile and .github/"
    Init,
    /// Copies a reusable module into the current project (timer, logger, api...)
    Import { target: ImportTarget },
    /// Calls ruff, isort and autoflake to format Python code
    #[command(name = "format-py")]
    FormatPy,
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
        Commands::Import { target } => match target {
            ImportTarget::Timer => Timer {}.deploy()?,
            ImportTarget::Logger => Logger {}.deploy()?,
            ImportTarget::Api => Api {}.deploy()?,
        },
        Commands::Init => Init {}.deploy()?,
        Commands::FormatPy => FormatPy {}.deploy()?,
    }
    Ok(())
}
