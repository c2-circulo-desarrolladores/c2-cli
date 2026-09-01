// use std::fs;

// use crate::core::Deployable;
// use crate::io::file_parser::FileParser;

// pub struct Init {
//     pub owner: Option<String>,
// }
// impl Init {
//     fn replace_in_pyproject(&self) -> std::io::Result<()> {
//         let mut pyproject_parser = FileParser::from(self.user_wd().join("pyproject.toml"))?;
//         let mut new_content = pyproject_parser
//             .contents
//             .replace("<REPO>", &self.repo_name())
//             .replace("<PACKAGE>", &self.package_name());

//         if let Some(owner) = &self.owner {
//             new_content = new_content.replace("<OWNER>", owner);
//             println!("✓ Replaced '<OWNER>' with {} in pyproject.toml", owner)
//         }

//         pyproject_parser.replace_content(new_content)?;
//         println!(
//             "✓ Replace '<REPO>' with {} in pyproject.toml",
//             &self.repo_name()
//         );

//         Ok(())
//     }

//     fn create_embedded_folders(&self) -> std::io::Result<()> {
//         let dir_name = self
//             .user_wd()
//             .file_name()
//             .unwrap()
//             .to_string_lossy()
//             .into_owned()
//             .replace("-", "_");
//         let folders = format!("src/{dir_name}");
//         let package_folder = self.user_wd().join(folders);
//         fs::create_dir_all(&package_folder)?;
//         fs::create_dir(self.user_wd().join("tests"))?;

//         let init_py_path = package_folder.join("__init__.py");
//         fs::write(init_py_path, b"__version__ = \"0.1.0\"\n")?;
//         Ok(())
//     }
// }

// impl Deployable for Init {
//     fn name(&self) -> &str {
//         "init"
//     }

//     fn deploy(&self) -> std::io::Result<()> {
//         self.cmd().execute("git", &["init"])?;
//         self.cmd().execute("rv.exe", &["init"])?;
//         self.cmd().execute(
//             "rv.exe",
//             &[
//                 "configure",
//                 "repository",
//                 "add",
//                 "--url",
//                 "https://cloud.r-project.org CRAN]",
//             ],
//         )?;
//         self.import_files()?;
//         self.replace_in_pyproject()?;
//         self.cmd().init()?;
//         self.create_embedded_folders()?;
//         println!("✓ Initialized project with .gitignore, cliff.toml, justfile and .github/");
//         Ok(())
//     }
// }
