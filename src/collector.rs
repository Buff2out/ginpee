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
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            let relative_path = entry.path().strip_prefix(base_path).unwrap();
            
            patterns.iter().any(|p| {
                // Сначала проверяем полный путь
                if p.matches_path(relative_path) {
                    return true;
                }
                
                // Для паттернов без wildcards проверяем basename
                let pattern_str = p.as_str();
                if !pattern_str.contains('*') && !pattern_str.contains('?') && !pattern_str.contains('[') {
                    if let Some(file_name) = entry.path().file_name() {
                        return file_name == pattern_str;
                    }
                }
                
                false
            })
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

    #[test]
    fn test_collect_files_without_extension() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir_all(temp_dir.path().join("subdir")).unwrap();
        
        let file1 = temp_dir.path().join("config");
        let file2 = temp_dir.path().join("subdir/config");
        fs::write(&file1, "config1").unwrap();
        fs::write(&file2, "config2").unwrap();

        let files = collect_files(temp_dir.path(), &["**/config".to_string()], ".gpskip").unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_collect_exact_filename() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir_all(temp_dir.path().join("nested")).unwrap();
        
        let file1 = temp_dir.path().join("Cargo.toml");
        let file2 = temp_dir.path().join("nested/other.toml");
        fs::write(&file1, "[package]").unwrap();
        fs::write(&file2, "[package]").unwrap();

        let files = collect_files(temp_dir.path(), &["Cargo.toml".to_string()], ".gpskip").unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("Cargo.toml"));
    }
}
