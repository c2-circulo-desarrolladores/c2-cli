use crate::deployables::base::Deployable;
use crate::io::file_parser::FileParser;

pub struct Init {}
impl Init {
    fn write_to_toml(&self) -> std::io::Result<()> {
        let mut pyproject_parser = FileParser::from(self.user_wd().join("pyproject.toml"))?;
        let found = pyproject_parser.search_for_line("[tool.commitizen]");
        if found == false {
            let version_vec = pyproject_parser.get_lines("version =");
            let version = version_vec
                .iter()
                .next()
                .expect("No version found in pyproject.toml")
                .split('"')
                .nth(1)
                .unwrap();
            let commitizen_block = format!(r#"
[tool.commitizen]
name = "cz_conventional_commits"
version = "{version}"
version_files = ["pyproject.toml:version"]
tag_format = "v$version""#
            );
            pyproject_parser.append_to_file(&commitizen_block)?;
        }
        println!("✓ Written '[tool.commitizen] block to pyproject.toml'");
        Ok(())
    }
}

impl Deployable for Init {
    fn name(&self) -> &str {
        "init"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.execute_command("init")?;
        self.import_files()?;
        self.write_to_toml()?;
        println!("✓ Initialized project with .gitignore, cliff.toml, justfile and .github/");
        Ok(())
    }
}
