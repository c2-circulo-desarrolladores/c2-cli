use std::{io, path::PathBuf, process::Command};

pub struct Commander {
    cwd: PathBuf,
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

    pub fn release(&self) -> io::Result<()> {
        self.execute("uv", &["run", "cz", "bump"])?;
        self.execute("uv", &["run", "git-cliff", "-o", "CHANGELOG.md"])?;
        self.execute("git", &["add", "CHANGELOG.md"])?;
        self.execute(
            "git",
            &["commit", "-m", "chore(changelog): update changelog"],
        )?;
        self.execute("git", &["push", "origin", "main"])?;
        self.execute("git", &["push", "origin", "--tags"])?;

        Ok(())
    }

    pub fn format_py(&self) -> io::Result<()> {
        self.execute("uv", &["run", "ruff", "check", "--fix", "."])
    }
}
