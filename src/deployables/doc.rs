use crate::core::Deployable;
use crate::io::FileParser;

pub struct Doc {
    pub owner: Option<String>,
}

impl Doc {
    fn replace_in_mkdocs(&self) -> std::io::Result<()> {
        let mut mkdocs_parser = FileParser::from(self.user_wd().join("mkdocs.yml"))?;
        let mut new_content = mkdocs_parser.contents.replace("<REPO>", &self.repo_name());
        if let Some(owner) = &self.owner {
            new_content = new_content.replace("<OWNER>", owner);
            println!("✓ Replaced '<OWNER>' with {} in mkdocs.yml", owner);
        }
        mkdocs_parser.replace_content(new_content)?;
        println!(
            "✓ Replaced '<REPO>' with {} in mkdocs.yml",
            &self.repo_name()
        );
        Ok(())
    }
}

impl Deployable for Doc {
    fn name(&self) -> &str {
        "doc"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.cmd().execute(
            "uv",
            &["add", "--dev", "mkdocs-material", "mkdocs_gen_files"],
        )?;
        self.import_files()?;
        self.replace_in_mkdocs()?;

        Ok(())
    }
}
