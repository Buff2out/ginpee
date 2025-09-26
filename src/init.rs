use std::fs;
use std::path::Path;

const DEFAULT_GPSKIP_CONTENT: &str = r#"
.git/
.vscode/
.idea/
target/
build/
dist/
node_modules/
*.log
*.tmp
*.swp
*.bak
.DS_Store
Thumbs.db
"#;

const DEFAULT_GINPEE_TOML_CONTENT: &str = r#"
[top]
text = ""

[down]
text = ""

[files]
include = []
"#;

pub fn run(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut created = 0;

    // ginpee.toml
    let toml_path = Path::new("ginpee.toml");
    if toml_path.exists() && !force {
        println!("ginpee.toml already exists. Use --force to overwrite.");
    } else {
        fs::write(toml_path, DEFAULT_GINPEE_TOML_CONTENT)?;
        println!("Created ginpee.toml");
        created += 1;
    }

    // .gpskip
    let gpskip_path = Path::new(".gpskip");
    let mut gpskip_content = DEFAULT_GPSKIP_CONTENT.to_string();

    if Path::new(".gitignore").exists() {
        let gitignore_content = fs::read_to_string(".gitignore")?;
        gpskip_content.push_str(&gitignore_content);
    }

    if gpskip_path.exists() && !force {
        println!(".gpskip already exists. Use --force to overwrite.");
    } else {
        fs::write(gpskip_path, gpskip_content)?;
        println!("Created .gpskip");
        created += 1;
    }

    println!("Total files created: {}", created);
    if created == 0 {
        println!("Nothing to create. Use --force to overwrite existing files.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_files() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        run(false).unwrap();

        assert!(Path::new("ginpee.toml").exists());
        assert!(Path::new(".gpskip").exists());
    }

    #[test]
    fn test_init_with_gitignore() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        fs::write(".gitignore", "node_modules/\n*.tmp").unwrap();
        run(false).unwrap();

        let gpskip = fs::read_to_string(".gpskip").unwrap();
        assert!(gpskip.contains("node_modules/"));
        assert!(gpskip.contains("*.tmp"));
    }
}