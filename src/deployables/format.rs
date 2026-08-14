use crate::core::Deployable;

pub struct Format {}

impl Deployable for Format {
    fn name(&self) -> &str {
        "format"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.cmd().format_py()?;
        Ok(())
    }
}
