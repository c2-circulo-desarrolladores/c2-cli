use crate::core::Deployable;

pub struct Polars {}

impl Deployable for Polars {
    fn name(&self) -> &str {
        "polars"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.import_files()?;
        Ok(())
    }
}
