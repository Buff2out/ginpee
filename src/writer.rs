//! Модуль записи результата в `project.md`.
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::formatter::FileContent;

pub fn write_project_md(
    output_path: &std::path::Path,
    top: Option<String>,
    tree: &str,
    contents: &[FileContent],
    down: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    if let Some(text) = top {
        writeln!(writer, "{}", text)?;
    }

    writeln!(writer, "# Project Structure\n")?;
    writeln!(writer, "```\n{}\n```", tree)?;

    for item in contents {
        writeln!(writer, "\n## File: `{}`\n", item.path)?;
        writeln!(writer, "```")?;
        writeln!(writer, "{}", item.content)?;
        writeln!(writer, "```")?;
    }

    if let Some(text) = down {
        writeln!(writer, "\n{}", text)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_project_md() {
        let temp_file = NamedTempFile::new().unwrap();
        let contents = vec![FileContent {
            path: "main.rs".to_string(),
            content: "// test".to_string(),
        }];

        write_project_md(
            temp_file.path(),
            Some("Top text".to_string()),
            "├── main.rs",
            &contents,
            Some("Down text".to_string()),
        )
        .unwrap();

        let output = fs::read_to_string(temp_file.path()).unwrap();
        assert!(output.contains("Top text"));
        assert!(output.contains("main.rs"));
        assert!(output.contains("// test"));
        assert!(output.contains("Down text"));
    }
}