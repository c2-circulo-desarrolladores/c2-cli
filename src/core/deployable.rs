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
    fn message(&self) -> &str;

    fn user_wd(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }
    fn project_root(&self) -> &PathBuf {
        &PROJECT_ROOT
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

    fn execute_command(&self, command_str: &str) -> io::Result<()> {
        let commands_vec: Vec<&str> = command_str.split(" ").collect();
        let (program, args) = commands_vec
            .split_first()
            .ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Lista vacía"))?;
        Command::new(program).args(args).output()?;
        println!("Ran {:?} command", command_str);
        Ok(())
    }

    fn import_files(&self) -> std::io::Result<()> {
        let src = self.folder_path()?;
        let target = self.user_wd()?;

        copy_dir(&src, &target)?;
        println!("{}", Self::message(&self));

        Ok(())
    }
    fn deploy(&self) -> std::io::Result<()> {
        self.import_files()?;
        Ok(())
    }
}
