use std::fs;
use std::path::PathBuf;

// Struct to read and modify contents of a File
pub struct FileParser {
    pub file_path: PathBuf,
    pub contents: String,
}

impl FileParser {
    pub fn from(path: PathBuf) -> std::io::Result<Self> {
        let contents = fs::read_to_string(&path)?;
        Ok(Self {
            file_path: path,
            contents,
        })
    }

    pub fn search_for_line(&self, search_for: &str) -> bool {
        let found = self.contents.lines().any(|x| x.trim().contains(search_for));
        found
    }

    pub fn get_lines(&self, search_for: &str) -> Vec<&str> {
        let lines: Vec<&str> = self
            .contents
            .lines()
            .filter(|x| x.contains(search_for))
            .collect();
        lines
    }

    pub fn append_to_file(&mut self, contents: &str) -> std::io::Result<()> {
        self.contents.push_str(contents);
        fs::write(&self.file_path, &self.contents)?;
        Ok(())
    }

    pub fn replace_content(&mut self, contents: String) -> std::io::Result<()> {
        fs::write(&self.file_path, &contents)?;
        self.contents = contents;
        Ok(())
    }
}
