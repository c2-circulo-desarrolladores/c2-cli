use crate::deployables::base::Deployable;
use crate::io::file_parser::FileParser;

pub struct Init {}
impl Init {
    fn write_to_toml(&self) -> std::io::Result<()> {
        let mut pyproject_parser = FileParser::from(self.user_wd().join("pyproject.toml"))?;
        let package_name = self
            .user_wd()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace("-", "_");
        let hatchling_block = format!(
            r#"
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatch.build.targets.wheel]
packages = ["{package_name}"]
"#,
        );
        pyproject_parser.append_to_file(&hatchling_block)?;
        println!("✓ Written 'hatchling' block to pyproject.toml");

        let ruff_block = r#"
[tool.ruff.lint]
select = ["F", "I", "E"]
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
