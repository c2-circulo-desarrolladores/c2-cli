use std::{
    fs,
    path::PathBuf,
};

use crate::core::Commander;
use include_dir::{Dir, include_dir};

pub static FOLDERS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/folders");

pub trait Deployable {
    fn name(&self) -> &str;

    fn cmd(&self) -> Commander {
        return Commander::new(self.user_wd());
    }

    fn user_wd(&self) -> PathBuf {
        return std::env::current_dir().unwrap();
    }

    fn folder(&self) -> &'static Dir<'static> {
        FOLDERS.get_dir(self.name()).expect("Folder not found")
    }

    fn import_files(&self) -> std::io::Result<()> {
        for file in self.folder().files() {
            let dest = self.user_wd().join(file.path());
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(dest, file.contents())?;
        }
        Ok(())
    }

    fn deploy(&self) -> std::io::Result<()>;
}
