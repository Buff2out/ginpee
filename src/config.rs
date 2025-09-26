use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub top: Option<Content>,
    pub down: Option<Content>,
    pub files: Option<Files>,
}

#[derive(Deserialize, Debug)]
pub struct Files {
    pub include: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    pub text: String,
}

pub fn load(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file
            .write_all(
                r#"
                [top]
                text = "Top text"
                [down]
                text = "Down text"
                [files]
                include = ["*.rs"]
                "#
                .as_bytes(),
            )
            .unwrap();

        let config = load(temp_file.path()).unwrap();
        assert_eq!(config.top.unwrap().text, "Top text");
        assert_eq!(config.down.unwrap().text, "Down text");
    }
}