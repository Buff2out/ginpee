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
fn test_nixos_config_structure() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Создаём полную структуру как в вашем nix-config
    fs::create_dir_all("dotfiles/helix/themes").unwrap();
    fs::create_dir_all("dotfiles/hypr").unwrap();
    fs::create_dir_all("dotfiles/kitty").unwrap();
    fs::create_dir_all("dotfiles/mako").unwrap();
    fs::create_dir_all("dotfiles/waybar").unwrap();
    fs::create_dir_all("home/modules").unwrap();
    fs::create_dir_all("hosts/nxos/modules").unwrap();
    fs::create_dir_all("overlays").unwrap();
    fs::create_dir_all("scripts/wallengine-rs/src").unwrap();

    // Создаём все файлы из вашего ginpee.toml
    // dotfiles/helix
    fs::write("dotfiles/helix/config.toml", "theme = \"synthwave\"").unwrap();
    fs::write("dotfiles/helix/themes/synthwave.toml", "[palette]\nprimary = \"#ff00ff\"").unwrap();

    // dotfiles/hypr
    fs::write("dotfiles/hypr/hyprland.conf", "# Hyprland configuration\ngeneral {\n  gaps_in = 5\n}").unwrap();
    fs::write("dotfiles/hypr/hyprlock.conf", "# Hyprlock config").unwrap();
    fs::write("dotfiles/hypr/hyprpaper.conf", "# Hyprpaper config").unwrap();

    // dotfiles/kitty
    fs::write("dotfiles/kitty/kitty.conf", "# Kitty terminal config\nfont_size 12.0").unwrap();

    // dotfiles/mako (без расширения!)
    fs::write("dotfiles/mako/config", "# Mako notification config\nmax-visible=5").unwrap();

    // dotfiles/waybar
    fs::write("dotfiles/waybar/config", "{\n  \"layer\": \"top\",\n  \"position\": \"top\"\n}").unwrap();
    fs::write("dotfiles/waybar/style.css", ".waybar {\n  background: #1e1e2e;\n}").unwrap();

    // home/modules
    fs::write("home/home.nix", "{ config, pkgs, ... }:\n{\n  home.username = \"wave\";\n}").unwrap();
    fs::write("home/modules/aliases.nix", "{ ll = \"ls -la\"; }").unwrap();
    fs::write("home/modules/cli-tools.nix", "{ pkgs }: [ pkgs.git ]").unwrap();
    fs::write("home/modules/env.nix", "{ EDITOR = \"hx\"; }").unwrap();
    fs::write("home/modules/shell.nix", "{ pkgs }: { programs.bash.enable = true; }").unwrap();
    fs::write("home/modules/vscode.nix", "{ programs.vscode.enable = true; }").unwrap();

    // hosts/nxos/modules
    fs::write("hosts/nxos/configuration.nix", "{ config, pkgs, ... }:\n{\n  imports = [ ./hardware-configuration.nix ];\n}").unwrap();
    fs::write("hosts/nxos/modules/audio.nix", "{ sound.enable = true; }").unwrap();
    fs::write("hosts/nxos/modules/battery.nix", "{ services.upower.enable = true; }").unwrap();
    fs::write("hosts/nxos/modules/bluetooth.nix", "{ hardware.bluetooth.enable = true; }").unwrap();
    fs::write("hosts/nxos/modules/desktop.nix", "{ services.xserver.enable = true; }").unwrap();
    fs::write("hosts/nxos/modules/fonts.nix", "{ fonts.packages = []; }").unwrap();
    fs::write("hosts/nxos/modules/gaming.nix", "{ programs.steam.enable = true; }").unwrap();
    fs::write("hosts/nxos/modules/locale.nix", "{ i18n.defaultLocale = \"en_US.UTF-8\"; }").unwrap();
    fs::write("hosts/nxos/modules/networking.nix", "{ networking.hostName = \"nxos\"; }").unwrap();
    fs::write("hosts/nxos/modules/nvidia.nix", "{ services.xserver.videoDrivers = [\"nvidia\"]; }").unwrap();
    fs::write("hosts/nxos/modules/packages.nix", "{ environment.systemPackages = []; }").unwrap();
    fs::write("hosts/nxos/modules/virtualisation.nix", "{ virtualisation.docker.enable = true; }").unwrap();
    fs::write("hosts/nxos/modules/wallpaper.nix", "{ services.hyprpaper.enable = true; }").unwrap();

    // scripts
    fs::write("scripts/cdev.sh", "#!/bin/bash\necho 'C development environment'").unwrap();
    fs::write("scripts/cppdev.sh", "#!/bin/bash\necho 'C++ development environment'").unwrap();
    fs::write("scripts/rsdev.sh", "#!/bin/bash\necho 'Rust development environment'").unwrap();
    fs::write("scripts/toggle-nightlight.sh", "#!/bin/bash\nhyprsunset -t 4500").unwrap();

    // scripts/wallengine-rs
    fs::write("scripts/wallengine-rs/Cargo.toml", "[package]\nname = \"wallengine-rs\"\nversion = \"0.1.0\"\nedition = \"2021\"").unwrap();
    fs::write("scripts/wallengine-rs/src/main.rs", "fn main() {\n    println!(\"Wallpaper engine\");\n}").unwrap();

    // Root files
    fs::write("flake.nix", "{\n  description = \"NixOS configuration\";\n  inputs = {};\n  outputs = {};\n}").unwrap();

    // ginpee.toml с вашим форматом (исправил опечатку "inclu" -> "include")
    fs::write(
        "ginpee.toml",
        r#"
[top]
text = ""

[down]
text = ""

[files]
include = [
    "synthwave.toml",
    "config.toml",
    "hyprland.conf",
    "hyprlock.conf",
    "hyprpaper.conf",
    "kitty.conf",
    "config",
    "style.css",
    "aliases.nix",
    "cli-tools.nix",
    "env.nix",
    "shell.nix",
    "vscode.nix",
    "home.nix",
    "audio.nix",
    "battery.nix",
    "bluetooth.nix",
    "desktop.nix",
    "fonts.nix",
    "gaming.nix",
    "locale.nix",
    "networking.nix",
    "nvidia.nix",
    "packages.nix",
    "virtualisation.nix",
    "wallpaper.nix",
    "configuration.nix",
    "main.rs",
    "Cargo.toml",
    "cdev.sh",
    "cppdev.sh",
    "rsdev.sh",
    "toggle-nightlight.sh",
    "flake.nix",
]
"#,
    )
    .unwrap();

    // Запускаем ginpee
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

    // Проверяем файлы helix
    assert!(output.contains("synthwave.toml"), "Missing synthwave.toml");
    assert!(output.contains("config.toml"), "Missing config.toml");
    assert!(output.contains("#ff00ff"), "Missing synthwave.toml content");

    // Проверяем hypr конфиги
    assert!(output.contains("hyprland.conf"), "Missing hyprland.conf");
    assert!(output.contains("hyprlock.conf"), "Missing hyprlock.conf");
    assert!(output.contains("hyprpaper.conf"), "Missing hyprpaper.conf");
    assert!(output.contains("gaps_in"), "Missing hyprland.conf content");

    // Проверяем kitty
    assert!(output.contains("kitty.conf"), "Missing kitty.conf");
    assert!(output.contains("font_size"), "Missing kitty.conf content");

    // Проверяем файлы без расширений
    assert!(output.contains("config"), "Missing config files");
    assert!(output.contains("max-visible"), "Missing mako config content");
    assert!(output.contains("\"layer\""), "Missing waybar config content");

    // Проверяем waybar
    assert!(output.contains("style.css"), "Missing style.css");
    assert!(output.contains(".waybar"), "Missing style.css content");

    // Проверяем home модули
    assert!(output.contains("home.nix"), "Missing home.nix");
    assert!(output.contains("aliases.nix"), "Missing aliases.nix");
    assert!(output.contains("cli-tools.nix"), "Missing cli-tools.nix");
    assert!(output.contains("env.nix"), "Missing env.nix");
    assert!(output.contains("shell.nix"), "Missing shell.nix");
    assert!(output.contains("vscode.nix"), "Missing vscode.nix");
    assert!(output.contains("home.username"), "Missing home.nix content");

    // Проверяем system модули
    assert!(output.contains("configuration.nix"), "Missing configuration.nix");
    assert!(output.contains("audio.nix"), "Missing audio.nix");
    assert!(output.contains("battery.nix"), "Missing battery.nix");
    assert!(output.contains("bluetooth.nix"), "Missing bluetooth.nix");
    assert!(output.contains("desktop.nix"), "Missing desktop.nix");
    assert!(output.contains("fonts.nix"), "Missing fonts.nix");
    assert!(output.contains("gaming.nix"), "Missing gaming.nix");
    assert!(output.contains("locale.nix"), "Missing locale.nix");
    assert!(output.contains("networking.nix"), "Missing networking.nix");
    assert!(output.contains("nvidia.nix"), "Missing nvidia.nix");
    assert!(output.contains("packages.nix"), "Missing packages.nix");
    assert!(output.contains("virtualisation.nix"), "Missing virtualisation.nix");
    assert!(output.contains("wallpaper.nix"), "Missing wallpaper.nix");
    assert!(output.contains("sound.enable"), "Missing audio.nix content");

    // Проверяем скрипты
    assert!(output.contains("cdev.sh"), "Missing cdev.sh");
    assert!(output.contains("cppdev.sh"), "Missing cppdev.sh");
    assert!(output.contains("rsdev.sh"), "Missing rsdev.sh");
    assert!(output.contains("toggle-nightlight.sh"), "Missing toggle-nightlight.sh");
    assert!(output.contains("hyprsunset"), "Missing toggle-nightlight.sh content");

    // Проверяем wallengine-rs
    assert!(output.contains("main.rs"), "Missing main.rs");
    assert!(output.contains("Cargo.toml"), "Missing Cargo.toml");
    assert!(output.contains("Wallpaper engine"), "Missing main.rs content");
    assert!(output.contains("wallengine-rs"), "Missing Cargo.toml content");

    // Проверяем flake.nix
    assert!(output.contains("flake.nix"), "Missing flake.nix");
    assert!(output.contains("NixOS configuration"), "Missing flake.nix content");

    println!("✅ All NixOS config files found and validated!");
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
