use crate::core::Deployable;
use crate::io::FileParser;

pub struct DocSphinx {
    pub owner: Option<String>,
}

impl DocSphinx {
    fn replace_in_doc(&self) -> std::io::Result<()> {
        let docs_path = self.user_wd().join("docs");
        let mut conf_parser = FileParser::from(docs_path.join("conf.py"))?;
        let mut new_content = conf_parser.contents.replace("<REPO>", &self.repo_name());
        if let Some(owner) = &self.owner {
            new_content = new_content.replace("<OWNER>", owner);
            println!("✓ Replaced '<OWNER>' with {} in conf.py", owner);
        }
        conf_parser.replace_content(new_content)?;
        println!("✓ Replaced '<REPO>' with {} in conf.py", &self.repo_name());
        Ok(())
    }
}

impl Deployable for DocSphinx {
    fn name(&self) -> &str {
        "doc"
    }

    fn deploy(&self) -> std::io::Result<()> {
        self.cmd()
            .execute("sphinx-autobuild", &["docs/source", "docs/build/html"])?;
        self.import_files()?;
        self.replace_in_doc()?;

        Ok(())
    }
}
