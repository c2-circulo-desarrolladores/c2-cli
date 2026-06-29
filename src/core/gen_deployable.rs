use crate::core::deployable::Deployable;

pub struct Init {}
impl Deployable for Init {
    fn name(&self) -> &str {
        "init"
    }
    fn message(&self) -> &str {
        "Initialized project with .gitignore, justfile and .github/"
    }
    fn deploy(&self) -> std::io::Result<()> {
        self.execute_command("uv init")?;
        self.import_files()?;
        self.execute_command("uv add --dev isort autoflake ruff pre-commit pytest")?;
        Ok(())
    }
}

pub struct FormatPy {}
impl Deployable for FormatPy {
    fn name(&self) -> &str {
        "format-py"
    }
    fn message(&self) -> &str {
        "Calls ruff, isort and autoflake to format Python code"
    }
    fn deploy(&self) -> std::io::Result<()> {
        self.execute_command("uv run autoflake --in-place --remove-unused-variables --remove-all-unused-imports -r . --exclude '__init__.py'")?;
        self.execute_command("uv run isort . --profile black")?;
        self.execute_command("uv run ruff check --fix . --exit-zero")?;
        Ok(())
    }
}

pub struct Timer {}
impl Deployable for Timer {
    fn name(&self) -> &str {
        "timer"
    }
    fn message(&self) -> &str {
        "Added measure_time.py"
    }
}
pub struct Logger {}
impl Deployable for Logger {
    fn name(&self) -> &str {
        "logger"
    }
    fn message(&self) -> &str {
        "Added ensure_logger.py"
    }
}

pub struct Api {}
impl Deployable for Api {
    fn name(&self) -> &str {
        "api"
    }
    fn message(&self) -> &str {
        "Added _api_wrapper.py and _async_api_wrapper.py"
    }
}
