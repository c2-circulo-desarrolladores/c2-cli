use std::{fs, path::PathBuf};

pub fn convert_rmd_to_r(path: &PathBuf) {
    let mut target_path = path.clone();
    target_path.set_extension("r");

    let contents = fs::read_to_string(path).unwrap();
    let contents_iter: Vec<_> = contents
        .split("\n")
        .filter(|x| !x.starts_with("```"))
        .collect();
    let contents_clean = contents_iter.join("\n");

    fs::write(&target_path, contents_clean).unwrap();
    println!(
        "Se convirtió {} a {:?}",
        path.file_name().unwrap().to_string_lossy(),
        target_path
    )
}
