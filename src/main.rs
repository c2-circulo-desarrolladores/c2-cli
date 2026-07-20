use c2_cli::core::Config;
use c2_cli::core::Deployable;
use c2_cli::deployables::executables::{Format, Release};
use c2_cli::deployables::importables::{Api, Logger, Timer};
use c2_cli::deployables::init::Init; // ajustá el path según dónde lo pongas

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

    /// Get or set persistent CLI configuration (e.g. default owner)
    Config {
        /// Set the default GitHub owner/org
        #[arg(long)]
        owner: Option<String>,

        /// Print the current config
        #[arg(long)]
        show: bool,
    },
}

#[derive(ValueEnum, Clone)]
enum ImportTarget {
    Timer,
    Logger,
    Api,
}

fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Init { owner } => {
            // Si no pasan --owner, cae al del config guardado
            let owner = match owner {
                Some(o) => Some(o),
                None => Config::load()?.owner,
            };
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

        Commands::Config { owner, show } => {
            let mut config = Config::load()?;

            if let Some(o) = owner {
                config.owner = Some(o);
                config.save()?;
                println!("Owner guardado: {}", config.owner.as_ref().unwrap());
            }

            if show || config.owner.is_none() {
                println!("{:#?}", config);
            }
        }
    }

    Ok(())
}
