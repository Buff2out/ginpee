use ginpee::run_command;
use serial_test::serial;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
#[serial]
fn test_find_makefile() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
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
        PathBuf::from("project.md"),
        None,
        None,
        vec![],
        PathBuf::from("ginpee.toml"),
    )
    .unwrap();

    // Проверяем, что project.md содержит Makefile
    let output = fs::read_to_string("project.md").unwrap();
    assert!(output.contains("Makefile"));
    assert!(output.contains("main.c"));

    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
#[serial]
fn test_nested_structure_with_exact_filenames() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Создаём вложенную структуру
    fs::create_dir_all("dotfiles/hypr").unwrap();
    fs::create_dir_all("dotfiles/mako").unwrap();
    fs::create_dir_all("dotfiles/waybar").unwrap();
    fs::create_dir_all("home/modules").unwrap();
    fs::create_dir_all("hosts/nxos/modules").unwrap();
    fs::create_dir_all("scripts").unwrap();

    // Создаём файлы
    fs::write("dotfiles/hypr/hyprland.conf", "# Hyprland config").unwrap();
    fs::write("dotfiles/mako/config", "# Mako config without extension").unwrap();
    fs::write("dotfiles/waybar/config", "# Waybar config").unwrap();
    fs::write("dotfiles/waybar/style.css", "/* CSS styles */").unwrap();
    fs::write("home/home.nix", "{ config, pkgs, ... }: {}").unwrap();
    fs::write("home/modules/shell.nix", "{ pkgs }: {}").unwrap();
    fs::write("hosts/nxos/configuration.nix", "{ config, ... }: {}").unwrap();
    fs::write("flake.nix", "{ description = \"test\"; }").unwrap();
    fs::write("Cargo.toml", "[package]\nname = \"test\"").unwrap();
    fs::write("scripts/rsdev.sh", "#!/bin/bash\necho test").unwrap();

    // ginpee.toml с паттернами
    fs::write(
        "ginpee.toml",
        r#"
[files]
include = [
    "**/hyprland.conf",
    "**/config",
    "**/style.css",
    "**/shell.nix",
    "**/home.nix",
    "**/configuration.nix",
    "flake.nix",
    "Cargo.toml",
    "**/*.sh"
]
"#,
    )
    .unwrap();

    run_command(
        PathBuf::from("project.md"),
        None,
        None,
        vec![],
        PathBuf::from("ginpee.toml"),
    )
    .unwrap();

    // Проверяем результат
    let output = fs::read_to_string("project.md").unwrap();
    
    assert!(output.contains("hyprland.conf"), "Missing hyprland.conf");
    assert!(output.contains("config"), "Missing config file");
    assert!(output.contains("style.css"), "Missing style.css");
    assert!(output.contains("shell.nix"), "Missing shell.nix");
    assert!(output.contains("home.nix"), "Missing home.nix");
    assert!(output.contains("configuration.nix"), "Missing configuration.nix");
    assert!(output.contains("flake.nix"), "Missing flake.nix");
    assert!(output.contains("Cargo.toml"), "Missing Cargo.toml");
    assert!(output.contains("rsdev.sh"), "Missing rsdev.sh");
    
    assert!(output.contains("# Hyprland config"));
    assert!(output.contains("# Mako config without extension"));

    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
#[serial]
fn test_basename_matching_for_files_without_extension() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    fs::create_dir_all("dir1").unwrap();
    fs::create_dir_all("dir2/subdir").unwrap();
    
    fs::write("dir1/config", "config1").unwrap();
    fs::write("dir2/subdir/config", "config2").unwrap();
    fs::write("config", "config_root").unwrap();

    fs::write(
        "ginpee.toml",
        r#"
[files]
include = ["**/config"]
"#,
    )
    .unwrap();

    run_command(
        PathBuf::from("project.md"),
        None,
        None,
        vec![],
        PathBuf::from("ginpee.toml"),
    )
    .unwrap();

    let output = fs::read_to_string("project.md").unwrap();
    
    assert!(output.contains("config1"));
    assert!(output.contains("config2"));
    assert!(output.contains("config_root"));

    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
#[serial]
fn test_exact_filename_vs_wildcard_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    fs::create_dir_all("src").unwrap();
    fs::write("src/main.rs", "fn main() {}").unwrap();
    fs::write("src/lib.rs", "pub fn test() {}").unwrap();
    fs::write("Cargo.toml", "[package]").unwrap();
    fs::write("README.md", "# Test").unwrap();

    fs::write(
        "ginpee.toml",
        r#"
[files]
include = [
    "**/*.rs",
    "Cargo.toml",
    "README.md"
]
"#,
    )
    .unwrap();

    run_command(
        PathBuf::from("project.md"),
        None,
        None,
        vec![],
        PathBuf::from("ginpee.toml"),
    )
    .unwrap();

    let output = fs::read_to_string("project.md").unwrap();
    
    assert!(output.contains("main.rs"));
    assert!(output.contains("lib.rs"));
    assert!(output.contains("Cargo.toml"));
    assert!(output.contains("README.md"));

    std::env::set_current_dir(original_dir).unwrap();
}
