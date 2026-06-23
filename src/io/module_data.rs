use std::path::PathBuf;

pub struct ModuleData {
    pub path: PathBuf,
    pub contents: String,
}

impl ModuleData {
    pub fn path_normalized(&self) -> String {
        self.path.to_string_lossy().replace("\\", "/")
    }

    pub fn path_comented(&self) -> String {
        format!("# {}", self.path_normalized())
    }
}
