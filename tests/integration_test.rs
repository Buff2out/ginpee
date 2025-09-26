use ginpee::run_command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_find_makefile() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Создаём файлы
    fs::write("main.c", "// test").unwrap();
    fs::write("Makefile", "all:\n\techo done").unwrap();

    // ginpee.toml
    fs::write(
        "ginpee.toml",
        r#"
[files]
include = ["*.c", "Makefile"]
"#,
    )
    .unwrap();

    run_command(
        std::path::PathBuf::from("project.md"),
        None,
        None,
        vec![],
        std::path::PathBuf::from("ginpee.toml"),
    )
    .unwrap();

    // Проверяем, что project.md содержит Makefile
    let output = fs::read_to_string("project.md").unwrap();
    assert!(output.contains("Makefile"));
    assert!(output.contains("main.c"));
}