use crate::deployables::base::Deployable;
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

    fn write_to_pyproject(&self) -> std::io::Result<()> {
        let mut pyproject_parser = FileParser::from(self.user_wd().join("pyproject.toml"))?;
        let package_name = self.package_name();
        let hatchling_block = format!(
            r#"
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatch.build.targets.wheel]
packages = ["src/{package_name}"]
"#,
        );
        pyproject_parser.append_to_file(&hatchling_block)?;
        println!("✓ Written 'hatchling' block to pyproject.toml");

        let ruff_block = r#"
[tool.ruff.lint]
select = ["F", "I", "E"]
ignore = ["E501"]
"#;
        pyproject_parser.append_to_file(&ruff_block)?;
        println!("✓ Written 'ruff' block to pyproject.toml");

        let version_line = pyproject_parser.get_lines("version =");
        let version_str = version_line
            .iter()
            .next()
            .expect("No version found in pyproject.toml")
            .split('"')
            .nth(1)
            .unwrap();
        let commitizen_block = format!(
            r#"
[tool.commitizen]
name = "cz_conventional_commits"
version = "{version_str}"
version_files = ["pyproject.toml:version"]
tag_format = "v$version""#
        );
        pyproject_parser.append_to_file(&commitizen_block)?;
        println!("✓ Written 'commitizen' block to pyproject.toml");

        Ok(())
    }

    fn write_to_cliff(&self) -> std::io::Result<()> {
        let mut cliff_parser = FileParser::from(self.user_wd().join("cliff.toml"))?;
        let mut new_content = cliff_parser.contents.replace("<REPO>", &self.dir_name());

        let mut placeholder = "";
        if let Some(owner) = &self.owner {
            new_content = new_content.replace("<OWNER>", owner);
            placeholder = "and <OWNER>";
        }

        cliff_parser.replace_content(new_content)?;

        println!(
            "✓ Replaced '<REPO>' {} in cliff.toml with {}",
            placeholder,
            &self.dir_name()
        );

        Ok(())
    }
}

impl Deployable for Init {
    fn name(&self) -> &str {
        "init"
    }

    fn deploy(&self) -> std::io::Result<()> {
        let dir_name = self
            .user_wd()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let folders = format!("src/{dir_name}");
        self.execute_just("init")?;
        self.import_files()?;
        self.write_to_pyproject()?;
        self.write_to_cliff()?;
        self.execute_just_with("mkdir", &[&folders])?;
        self.execute_just_with("mkdir", &[&"tests"])?;
        println!("✓ Initialized project with .gitignore, cliff.toml, justfile and .github/");
        Ok(())
    }
}
