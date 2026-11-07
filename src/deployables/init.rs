use std::fs;

use crate::core::Deployable;
use crate::io::file_parser::FileParser;

pub struct Init {
    pub owner: Option<String>,
}
impl Init {
    fn add_to_pyproject(&self) -> std::io::Result<()> {
        let mut pyproject_parser = FileParser::from(self.user_wd().join("pyproject.toml"))?;

        let owner = self.owner.as_deref().unwrap_or("<OWNER>");
        let repo = self.repo_name();

        // Blocks that belong before [build-system]
        let project_blocks = format!(
            r#"classifiers = [
    "Development Status :: 3 - Alpha",
    "Intended Audience :: Developers",
    "Intended Audience :: Education",
    "Intended Audience :: Science/Research",

    "Programming Language :: Python",
    "Programming Language :: Python :: 3",
    "Operating System :: OS Independent",
    "Typing :: Typed",

    "Topic :: Software Development :: Libraries",
    "Topic :: Software Development :: Libraries :: Python Modules",
    "Natural Language :: Spanish",
]

[project.urls]
Repository = "https://github.com/{owner}/{repo}"
Changelog = "https://github.com/{owner}/{repo}/blob/main/CHANGELOG.md"

[dependency-groups]
dev = ["pre-commit", "pytest", "ruff"]

"#
        );

        // `uv init` always writes a [build-system] table, but fall back to
        // appending when a project was created without a build backend.
        let index = pyproject_parser.contents.find("\n[build-system]").expect(
            "uv init did not generate a [build-system] block\nPlease update with 'uv self update'",
        );
        let (head, tail) = pyproject_parser.contents.split_at(index + 1);
        let contents = format!("{head}{project_blocks}{tail}");

        let tool_blocks = r#"[tool.commitizen]
name = "cz_conventional_commits"
version_provider = "uv"
version_scheme = "pep440"
tag_format = "v$version"
annotated_tag = true
bump_message = "bump: version $current_version → $new_version"
major_version_zero = true
post_bump_hooks = [
    "git push origin HEAD --follow-tags",
]

[tool.pytest.ini_options]
testpaths = ["tests"]

[tool.ruff.lint]
select = ["F", "I", "E"]
ignore = ["E501"]
"#;

        pyproject_parser.replace_content(format!("{}\n\n{}", contents.trim_end(), tool_blocks))?;
        println!("✓ Added classifiers, urls, dependency-groups and tool config to pyproject.toml");

        Ok(())
    }

    fn replace_in_cliff(&self) -> std::io::Result<()> {
        let repo_url = self
            .cmd()
            .execute_with_output("git", &["remote", "get-url", "origin"])
            .expect("No se pudo obtener el url del repositorio remoto 'origin'")
            .trim_end_matches(".git")
            .to_string();

        let mut cliff_parser = FileParser::from(self.user_wd().join("cliff.toml"))?;
        let mut new_content = cliff_parser.contents.replace("<REPO_URL>", &repo_url);

        if let Some(owner) = &self.owner {
            new_content = new_content.replace("<OWNER>", owner);
            println!("✓ Replaced '<OWNER>' with {} in cliff.toml", owner)
        }

        cliff_parser.replace_content(new_content)?;

        println!("✓ Replaced '<REPO_URL>' with {} in cliff.toml", repo_url);

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
        let contents = format!(
            r#"from importlib.metadata import PackageNotFoundError, version

try:
    __version__ = version("{}")
except PackageNotFoundError:
    __version__ = "unknown"
"#,
            self.package_name()
        );
        fs::write(init_py_path, contents)?;
        let py_typed_file = package_folder.join("py.typed");
        fs::write(py_typed_file, b"")?;
        Ok(())
    }
}

impl Deployable for Init {
    fn name(&self) -> &str {
        "init"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.cmd().execute("git", &["init"])?;
        self.cmd().uv_init(&self.repo_name())?;
        self.add_to_pyproject()?;
        self.import_files()?;
        self.replace_in_cliff()?;
        self.create_embedded_folders()?;
        self.cmd().execute("uv", &["sync"])?;
        println!("✓ Initialized project with .gitignore, cliff.toml, .pre-commit and .github/");
        Ok(())
    }
}
