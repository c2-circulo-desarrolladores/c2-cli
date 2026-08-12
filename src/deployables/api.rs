use crate::core::Deployable;

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
