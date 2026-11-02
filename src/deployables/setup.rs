use crate::core::Deployable;

pub struct Setup {}

impl Deployable for Setup {
    fn name(&self) -> &str {
        "setup"
    }

    fn deploy(&self) -> std::io::Result<()> {
        
        Ok(())
    }
}



// uv tool install bump-my-version
// winget install git-cliff
// mac: sudo port install git-cliff