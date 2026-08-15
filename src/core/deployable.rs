use std::{fs, path::PathBuf};

use crate::core::Commander;
use include_dir::{Dir, DirEntry, include_dir};

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

    fn repo_name(&self) -> String {
        return self
            .user_wd()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
    }

    fn package_name(&self) -> String {
        return self.repo_name().replace("-", "_");
    }

    fn import_files(&self) -> std::io::Result<()> {
        self.import_dir_recursive(self.folder())
    }

    fn import_dir_recursive(&self, dir: &Dir<'static>) -> std::io::Result<()> {
        for entry in dir.entries() {
            match entry {
                DirEntry::File(file) => {
                    let relative_path = file.path().strip_prefix(self.name()).unwrap();
                    let dest = self.user_wd().join(relative_path);
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(dest, file.contents())?;
                }
                DirEntry::Dir(subdir) => {
                    self.import_dir_recursive(subdir)?;
                }
            }
        }
        Ok(())
    }

    fn deploy(&self) -> std::io::Result<()>;
}
