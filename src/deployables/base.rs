use std::process::Command;
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::LazyLock,
};

pub static PROJECT_ROOT: LazyLock<PathBuf> = LazyLock::new(|| env!("CARGO_MANIFEST_DIR").into());

fn copy_dir(src: &Path, target: &Path) -> std::io::Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        let dest = target.join(entry.file_name());
        if path.is_file() {
            fs::copy(&path, &dest)?;
        }
        if path.is_dir() {
            copy_dir(&path, &dest)?;
        }
    }
    Ok(())
}

pub trait Deployable {
    fn name(&self) -> &str;

    fn user_wd(&self) -> PathBuf {
        return std::env::current_dir().unwrap();
    }

    fn project_root(&self) -> &PathBuf {
        &PROJECT_ROOT
    }

    fn justfile_path(&self) -> PathBuf {
        self.project_root().join("src").join("justfile")
    }

    fn folder_path(&self) -> io::Result<PathBuf> {
        let path = PROJECT_ROOT.join("src").join("folders").join(self.name());
        if path.exists() && path.is_dir() {
            Ok(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Folder {} not found at {:?}", self.name(), path),
            ))
        }
    }

    fn execute_just(&self, recipe: &str) -> io::Result<()> {
        self.execute_just_with(recipe, &[])?;
        Ok(())
    }

    fn execute_just_with(&self, recipe: &str, args: &[&str]) -> io::Result<()> {
        let justfile = self.justfile_path();
        let status = Command::new("just")
            .arg("--justfile")
            .arg(&justfile)
            .arg("--working-directory")
            .arg(&self.user_wd())
            .arg(recipe)
            .args(args)
            .status()?;
        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
        Ok(())
    }

    fn import_files(&self) -> std::io::Result<()> {
        let src = self.folder_path()?;
        let target = self.user_wd();

        copy_dir(&src, &target)?;
        Ok(())
    }

    // fn write_files(&self) -> () {
    //     todo!("No implementado")
    // }

    fn deploy(&self) -> std::io::Result<()>;
}
