use crate::deployables::base::Deployable;


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
