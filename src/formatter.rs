//! Модуль формирования дерева и содержимого файлов для `project.md`.
use std::fs;
use std::path::Path;

pub struct FileContent {
    pub path: String,
    pub content: String,
}

pub fn build_tree_and_content(
    files: &[std::path::PathBuf],
    base_path: &Path,
) -> Result<(String, Vec<FileContent>), Box<dyn std::error::Error>> {
    let mut tree = String::new();
    let mut contents = Vec::new();

    for file_path in files {
        let relative_path = file_path.strip_prefix(base_path)?;
        let path_str = relative_path.to_string_lossy().replace('\\', "/"); // для Windows
        tree.push_str(&format!("{}\n", path_str));

        let content = fs::read_to_string(file_path)?;
        contents.push(FileContent {
            path: path_str.to_string(),
            content,
        });
    }

    Ok((tree, contents))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_build_tree_and_content() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("main.rs");
        fs::write(&file1, "// test").unwrap();

        let files = vec![file1];
        let (tree, contents) = build_tree_and_content(&files, temp_dir.path()).unwrap();

        assert!(tree.contains("main.rs"));
        assert_eq!(contents[0].content, "// test");
    }
}