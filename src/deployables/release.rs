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
            Version::Patch => "PATCH",
            Version::Minor => "MINOR",
            Version::Major => "MAJOR",
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Prerelease {
    Alpha,
    Beta,
    Rc,
}

impl Prerelease {
    pub fn as_str(&self) -> &'static str {
        match self {
            Prerelease::Alpha => "alpha",
            Prerelease::Beta => "beta",
            Prerelease::Rc => "rc",
        }
    }
}

pub struct Release {
    pub part: Version,
    pub prerelease: Option<Prerelease>,
}

impl Deployable for Release {
    fn name(&self) -> &str {
        "release"
    }

    fn deploy(&self) -> std::io::Result<()> {
        let prerelease_parsed = self.prerelease.map(|p| p.as_str());
        self.cmd()
            .release(self.part.as_str(), prerelease_parsed, &self.user_wd())?;
        Ok(())
    }
}
