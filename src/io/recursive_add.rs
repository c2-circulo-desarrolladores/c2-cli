use crate::io::module_path::{add_module_path, read_file};
use std::path::Path;

pub fn recursive_add(root_dir: &Path, extension_filter: Option<&str>) {
    let entries = root_dir
        .read_dir()
        .unwrap_or_else(|e| panic!("Couldn't read entry {:?}: {}", root_dir, e));

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let matches = match extension_filter {
                Some(filter) => ext == filter,
                None => true,
            };
            if matches {
                    let contents = read_file(&path);
                    add_module_path(contents);
            }
        }
    }
}
