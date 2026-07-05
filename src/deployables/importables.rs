use crate::deployables::Deployable;

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
