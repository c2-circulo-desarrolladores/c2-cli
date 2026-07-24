use std::path::PathBuf;

use crate::core::Deployable;
use crate::io::FileParser;

use walkdir::WalkDir;

pub struct FixInits {}

impl FixInits {
    fn obtain_modules(line: &str) -> Vec<String> {
        let mut modules: Vec<String> = Vec::new();
        // Primer caso: una línea
        if line.contains(",") {
            let modules_list: Vec<String> = line
                .split("import")
                .last()
                .unwrap()
                .split(",")
                .map(|x| x.trim().to_string())
                .collect();
            modules.extend(modules_list.into_iter());
        } else {
            let module_str = line.split(" ").last().unwrap().to_string();
            modules.push(module_str);
            // Segundo caso: multilínea
            // if line.contains("(") {
        }
        modules
    }

    fn populate_all_inside_init(&self, mut file_parser: FileParser) -> std::io::Result<()> {
        let modules: Vec<String> = file_parser
            .contents
            .lines()
            .filter(|x| x.starts_with("import ") || x.starts_with("from "))
            .flat_map(Self::obtain_modules)
            .collect();

        let modules_str = modules
            .iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(", ");

        let all_block = &format!("__all__ = [{}]", modules_str);

        let has_all = file_parser.search_for_line("__all__");
        if !has_all {
            file_parser.append_to_file(all_block)?;
        } else {
            let old_all = file_parser
                .contents
                .lines()
                .filter(|x| x.starts_with("__all__"))
                .next()
                .unwrap();
            let new_content = file_parser.contents.replace(old_all, all_block);
            file_parser.replace_content(new_content)?;
        };
        Ok(())
    }
}

impl Deployable for FixInits {
    fn name(&self) -> &str {
        "fix_inits"
    }

    fn deploy(&self) -> std::io::Result<()> {
        let init_paths: Vec<walkdir::DirEntry> = WalkDir::new(self.user_wd())
            .into_iter()
            .filter_entry(|x| x.path().file_name().unwrap() != ".venv")
            .filter_map(|x| x.ok())
            .filter(|x| x.path().is_file())
            .filter(|x| x.file_name() == "__init__.py")
            .into_iter()
            .collect();
        for path in init_paths {
            let file_parser = FileParser::from(PathBuf::from(path.path()))?;
            self.populate_all_inside_init(file_parser)?;
            println!(
                "✓ Populated __all__ inside {}",
                path.path().to_string_lossy()
            )
        }
        Ok(())
    }
}
