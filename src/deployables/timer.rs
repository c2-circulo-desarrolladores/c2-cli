use crate::core::Deployable;

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
