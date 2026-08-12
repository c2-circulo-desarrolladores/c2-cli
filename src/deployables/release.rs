use crate::core::Deployable;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Version {
    Patch,
    Minor,
    Major,
}

impl Version {
    pub fn as_str(&self) -> &'static str {
        match self {
            Version::Patch => "patch",
            Version::Minor => "minor",
            Version::Major => "major",
        }
    }
}

pub struct Release {
    pub part: Version,
}

impl Deployable for Release {
    fn name(&self) -> &str {
        "release"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.cmd().release(&self.part.as_str())?;
        Ok(())
    }
}
