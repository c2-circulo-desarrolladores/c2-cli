use crate::core::deployable::Deployable;

pub struct Init {}
impl Deployable for Init {
    fn name(&self) -> &str {
        "init"
    }
    fn deploy(&self) -> std::io::Result<()> {
        self.execute_command("init")?;
        self.import_files()?;
        println!("✓ Initialized project with .gitignore, cliff.toml, justfile and .github/");
        Ok(())
    }
}

pub struct Release {}
impl Deployable for Release {
    fn name(&self) -> &str {
        "release"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.execute_command("release")?;
        Ok(())
    }
}

pub struct FormatPy {}
impl Deployable for FormatPy {
    fn name(&self) -> &str {
        "format-py"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.execute_command("format-py")?;
        Ok(())
    }
}

pub struct Timer {}
impl Deployable for Timer {
    fn name(&self) -> &str {
        "timer"
    }
    fn deploy(&self) -> std::io::Result<()> {
        self.import_files()?;
        Ok(())
    }
}

pub struct Logger {}
impl Deployable for Logger {
    fn name(&self) -> &str {
        "logger"
    }
    fn deploy(&self) -> std::io::Result<()> {
        self.import_files()?;
        Ok(())
    }
}

pub struct Api {}
impl Deployable for Api {
    fn name(&self) -> &str {
        "api"
    }
    fn deploy(&self) -> std::io::Result<()> {
        self.import_files()?;
        Ok(())
    }
}
