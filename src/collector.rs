//! Модуль сбора файлов с учётом `.gpskip` и фильтров.
use glob::Pattern;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub fn collect_files(
    base_path: &Path,
    include_patterns: &[String],
    ignore_file: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut walker = WalkBuilder::new(base_path);
    walker.add_custom_ignore_filename(ignore_file);
    let walker = walker.build();

    let mut files = Vec::new();

    for result in walker {
        let entry = result?;
        let path = entry.path();

        if path.is_dir() {
            eprintln!("Dir: {:?}", path);
        } else if path.is_file() {
            eprintln!("File: {:?}", path);
            if include_patterns.is_empty() {
                files.push(path.to_path_buf());
            } else {
                for pattern in include_patterns {
                    if Pattern::new(pattern)?.matches_path(path) {
                        files.push(path.to_path_buf());
                        break;
                    }
                }
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_collect_files_with_patterns() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("main.rs");
        let file2 = temp_dir.path().join("README.md");
        fs::write(&file1, "// test").unwrap();
        fs::write(&file2, "# Readme").unwrap();

        let files = collect_files(temp_dir.path(), &["*.rs".to_string()], ".gpskip").unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("main.rs"));
    }
}