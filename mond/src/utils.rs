use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub(crate) fn find_mond_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("mond"))
        .map(|e| e.path().to_path_buf())
        .collect();
    files.sort();
    files
}
