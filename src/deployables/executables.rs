use crate::core::Deployable;

pub struct Release {}
impl Deployable for Release {
    fn name(&self) -> &str {
        "release"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.cmd().release()?;
        Ok(())
    }
}

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
