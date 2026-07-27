use crate::core::Deployable;
use crate::deployables::polars::Extension::{Csv, Xlsx};
use crate::io::file_parser::FileParser;
use std::fs;

pub enum Extension {
    Csv,
    Xlsx,
}

pub struct Polars {
    extension: Extension,
    file_name: String,
}

impl Deployable for Polars {
    fn name(&self) -> &str {
        "polars"
    }
    fn deploy(&self) -> std::io::Result<()> {
        let importable_file = match self.extension {
            Csv => "csv_template.py",
            Xlsx => "xlsx_template.py",
        };
        for file in self.folder().files() {
            let relative_path = file.path().strip_prefix(self.name()).unwrap();
            if relative_path.file_name().unwrap() == importable_file {
                let dest = self.user_wd().join(relative_path);
                fs::write(dest, file.contents())?;
            }
        }
        let mut parser = FileParser::from(self.user_wd().join(importable_file))?;
        let new_content = parser.contents.replace("your_file", &self.file_name);
        parser.replace_content(new_content)?;

        Ok(())
    }
}
