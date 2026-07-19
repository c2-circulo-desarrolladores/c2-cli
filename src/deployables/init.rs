use std::fs;

use crate::core::Deployable;
use crate::io::file_parser::FileParser;

pub struct Init {
    pub owner: Option<String>,
}
impl Init {
    fn dir_name(&self) -> String {
        return self
            .user_wd()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
    }

    fn package_name(&self) -> String {
        return self.dir_name().replace("-", "_");
    }

    fn replace_in_pyproject(&self) -> std::io::Result<()> {
        let mut pyproject_parser = FileParser::from(self.user_wd().join("pyproject.toml"))?;
        let mut new_content = pyproject_parser
            .contents
            .replace("<REPO>", &self.dir_name())
            .replace("<PACKAGE>", &self.package_name());

        if let Some(owner) = &self.owner {
            new_content = new_content.replace("<OWNER>", owner);
            println!("✓ Replaced '<OWNER>' with {} in pyproject.toml", owner)
        }

        pyproject_parser.replace_content(new_content)?;
        println!(
            "✓ Replace '<REPO>' with {} in pyproject.toml",
            &self.dir_name()
        );

        Ok(())
    }

    fn replace_in_cliff(&self) -> std::io::Result<()> {
        let mut cliff_parser = FileParser::from(self.user_wd().join("cliff.toml"))?;
        let mut new_content = cliff_parser.contents.replace("<REPO>", &self.dir_name());

        if let Some(owner) = &self.owner {
            new_content = new_content.replace("<OWNER>", owner);
            println!("✓ Replaced '<OWNER>' with {} in cliff.toml", owner)
        }

        cliff_parser.replace_content(new_content)?;

        println!(
            "✓ Replaced '<REPO>' with {} in cliff.toml",
            &self.dir_name()
        );

        Ok(())
    }

    fn create_embedded_folders(&self) -> std::io::Result<()> {
        let dir_name = self
            .user_wd()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .replace("-", "_");
        let folders = format!("src/{dir_name}");
        let package_folder = self.user_wd().join(folders);
        fs::create_dir_all(&package_folder)?;
        fs::create_dir(self.user_wd().join("tests"))?;

        let init_py_path = package_folder.join("__init__.py");
        fs::write(init_py_path, b"")?;
        Ok(())
    }
}

impl Deployable for Init {
    fn name(&self) -> &str {
        "init"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.cmd().execute("git", &["init"])?;
        self.import_files()?;
        self.replace_in_pyproject()?;
        self.replace_in_cliff()?;
        self.cmd().init()?;
        self.create_embedded_folders()?;
        println!("✓ Initialized project with .gitignore, cliff.toml, justfile and .github/");
        Ok(())
    }
}
