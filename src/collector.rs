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
    walker.git_ignore(false);
    walker.git_global(false);
    walker.git_exclude(false);
    let walker = walker.build();

    let patterns: Result<Vec<_>, _> = include_patterns.iter()
        .map(|s| Pattern::new(s))
        .collect();

    let patterns = patterns?;

    let files: Vec<_> = walker
        .filter_map(|result| result.ok())
        .filter(|entry| {
            eprintln!("File found: {:?}", entry);
            entry.path().is_file()
        })
        .filter(|entry| {
            eprintln!("Trying pattern: {:?} on path: {:?}", patterns, entry);
            if patterns.is_empty() {
                true
            } else {
                patterns.iter().any(|p| p.matches_path(entry.path()))
            }
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

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