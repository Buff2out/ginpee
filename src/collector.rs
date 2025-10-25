//! Модуль сбора файлов с учётом `.gpskip` и фильтров.
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use globset::{Glob, GlobSetBuilder};

pub fn collect_files(
    base_path: &Path,
    include_patterns: &[String],
    ignore_file: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut builder = GlobSetBuilder::new();
    for pattern in include_patterns {
        builder.add(Glob::new(pattern)?);
    }
    let globset = builder.build()?;

    let mut walker = WalkBuilder::new(base_path);
    walker.add_custom_ignore_filename(ignore_file);
    walker.git_ignore(false);
    walker.git_global(false);
    walker.git_exclude(false);

    let files: Vec<_> = walker.build()
        .filter_map(|result| result.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            let relative_path = entry.path().strip_prefix(base_path).unwrap();
            globset.is_match(relative_path)
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