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
        self.execute_command("uv add --dev isort autoflake ruff pre-commit")?;
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
