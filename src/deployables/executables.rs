use crate::deployables::base::Deployable;

pub struct Release {}
impl Deployable for Release {
    fn name(&self) -> &str {
        "release"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.execute_just("release")?;
        Ok(())
    }
}

pub struct Format {}
impl Deployable for Format {
    fn name(&self) -> &str {
        "format"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.execute_just("format-py")?;
        Ok(())
    }
}
