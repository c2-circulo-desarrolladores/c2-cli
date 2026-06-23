use std::fs::{self};
use std::path::PathBuf;
use crate::io::module_data::ModuleData;

pub fn read_file(file_path: &PathBuf) -> ModuleData {
    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|e| panic!("Couldn't read the file {:?}: {}", file_path, e));
    return ModuleData {
        path: file_path.clone(),
        contents,
    };
}

/// Adds a module path on top of file.
/// If there is already a module path, replaces old path.
/// Recognizes a module path if the first line starts with '#' and has at least one '/'
/// 
pub fn add_module_path(mut file_data: ModuleData) -> () {
    let first_line = file_data.contents.split("\n").next().unwrap_or("");
    let rewrite_first = first_line.starts_with("#") && first_line.contains("/");

    file_data.contents = if rewrite_first {
        let rest = file_data.contents.splitn(2, "\n").nth(1).unwrap(); // This should not fail
        format!("{}\n{}", file_data.path_comented(), rest)
    } else {
        format!("{}\n{}", file_data.path_comented(), file_data.contents)
    };

    fs::write(&file_data.path, &file_data.contents)
        .unwrap_or_else(|e| panic!("Couldn't write to file {:?}: {}", &file_data.path, e));
}
