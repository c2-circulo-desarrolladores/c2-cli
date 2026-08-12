use std::{io, path::PathBuf, process::Command};

pub struct Commander {
    cwd: PathBuf,
}

fn parse_next_version(output: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let line = line.trim();
        line.strip_prefix("New version will be '")
            .and_then(|rest| rest.strip_suffix('\''))
            .map(|s| s.to_string())
    })
}

impl Commander {
    pub fn new(cwd: impl Into<PathBuf>) -> Self {
        Self { cwd: cwd.into() }
    }

    pub fn execute(&self, program: &str, args: &[&str]) -> io::Result<()> {
        let full_command = format!("{} {}", program, args.join(" "));
        println!("\x1b[32m====> Running:\x1b[0m {}", full_command);

        let status = Command::new(&program)
            .current_dir(&self.cwd)
            .args(args)
            .status()?;

        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
        Ok(())
    }

    pub fn execute_with_output(&self, program: &str, args: &[&str]) -> io::Result<String> {
        let out = Command::new(&program).args(args).output()?;
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    // -------------------------------------------------------------------------
    // Recipes
    // -------------------------------------------------------------------------

    pub fn init(&self) -> io::Result<()> {
        self.execute("uv", &["sync"])?;
        self.execute("uv", &["run", "pre-commit", "install"])?;

        Ok(())
    }

    pub fn init_r(&self) -> io::Result<()> {
        self.execute("git", &["init"])?;
        self.execute("rv", &["init"])?;
        self.execute(
            "rv",
            &[
                "add",
                "--dev",
                "devtools",
                "roxygen2",
                "testthat",
                "pkgdown",
                "knitr",
                "rmarkdown",
                "usethis",
            ],
        )?;

        Ok(())
    }

    pub fn release(&self, part: &str) -> io::Result<()> {
        // 1. Previsualizar próxima versión (dry-run, no toca nada)
        let dry_run_output = self.execute_with_output(
            "uv",
            &[
                "run",
                "bump-my-version",
                "bump",
                "--dry-run",
                "--verbose",
                "--allow-dirty",
                part,
            ],
        )?;
        let next_version = parse_next_version(&dry_run_output)
            .expect("no se pudo extraer la próxima versión del output de bump-my-version");

        // 2. Generar changelog con esa versión
        self.execute(
            "uv",
            &[
                "run",
                "git-cliff",
                "--unreleased",
                "--tag",
                &next_version,
                "--prepend",
                "CHANGELOG.md",
            ],
        )?;
        self.execute("git", &["add", "CHANGELOG.md"])?;
        self.execute(
            "git",
            &["commit", "-m", "chore(changelog): update changelog"],
        )?;

        self.execute("uv", &["run", "bump-my-version", "bump", part])?;
        self.execute("git", &["push", "origin", "main"])?;
        self.execute("git", &["push", "origin", "--tags"])?;
        Ok(())
    }

    pub fn format_py(&self) -> io::Result<()> {
        self.execute("uv", &["run", "ruff", "check", "--fix", "."])?;
        self.execute("uv", &["run", "ruff", "format", "."])?;
        Ok(())
    }
}
