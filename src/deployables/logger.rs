use crate::core::Deployable;

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
