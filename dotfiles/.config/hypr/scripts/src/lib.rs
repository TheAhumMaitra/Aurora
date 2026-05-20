// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//    Copyright (C) 2026 Ahum Maitra

//      This program is free software: you can redistribute it and/or modify
//      it under the terms of the GNU General Public License as published by
//      the Free Software Foundation, either version 3 of the License, or
//      (at your option) any later version.

//      This program is distributed in the hope that it will be useful,
//      but WITHOUT ANY WARRANTY; without even the implied warranty of
//      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//      GNU General Public License for more details.

//      You should have received a copy of the GNU General Public License
//      along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serde::Deserialize;
use std::fs;
use std::io::Read;

use std::path::{Path, PathBuf};
use std::process::Command;

use gtk4 as gtk;
use gtk4::CssProvider;
use gtk4::gdk::Display;

pub struct AuroraPaths {
    pub home: PathBuf,
    pub config: PathBuf,
    pub themes: PathBuf,
}

impl AuroraPaths {
    pub fn new() -> Self {
        let home = dirs::home_dir().expect("Could not get HOME directory");
        let config = home.join(".config");

        Self {
            themes: config.join("themes"),
            home,
            config,
        }
    }
}

pub fn aurora_paths() -> AuroraPaths {
    AuroraPaths::new()
}

#[derive(Debug, Clone)]
pub struct ThemeEntry {
    pub directory_name: String,
    pub display_name: String,
}

fn read_theme_config(theme_dir: &std::path::Path) -> Option<Config> {
    let config_path = theme_dir.join("config.toml");
    fs::read_to_string(config_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
}

pub fn theme_entries() -> Vec<ThemeEntry> {
    let paths = aurora_paths();
    let mut themes = Vec::new();

    if let Ok(entries) = fs::read_dir(&paths.themes) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let directory_name = path.file_name().unwrap().to_string_lossy().to_string();
            let display_name = read_theme_config(&path)
                .map(|config| config.name)
                .filter(|name| !name.trim().is_empty())
                .unwrap_or_else(|| directory_name.clone());

            themes.push(ThemeEntry {
                directory_name,
                display_name,
            });
        }
    }

    themes.sort_by(|a, b| {
        a.display_name
            .to_lowercase()
            .cmp(&b.display_name.to_lowercase())
    });
    themes
}

// list all available themes in `~/.config/themes` for mostly cli
pub fn list_themes() {
    println!("Showing Available Themes:\n");

    for theme in theme_entries() {
        println!("• {}", theme.display_name);
    }
}

// Root structure for parsing config.toml
#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    version: String,
    authors: Vec<String>,
    repo_url: Option<String>,
    wallpapers_sources: Option<Vec<String>>,
    license: Option<String>,
    settings: Option<Settings>,
}

// get options inside of the toml file (we only need required options)
#[derive(Deserialize, Debug)]
struct Settings {
    script: Option<String>,
    interpreter: Option<String>,
}

// apply the selected theme
pub fn apply_theme(theme_name: &str) {
    let paths = aurora_paths();
    let aurora_data = paths.home.join(".local/share/Aurora");
    let hypr_theme_file = paths.config.join("hypr/Theme/theme.lua");

    // get the `config.toml` from the selected theme
    let mut config_path = paths.themes.clone();
    config_path.push(theme_name);
    config_path.push("config.toml");

    let config: Option<Config> = fs::read_to_string(&config_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok());

    let folders = ["waybar", "wlogout", "hypr", "rofi", "nvim", "btop"]; //directories want to be copied

    let message = format!("{theme_name} is applied!");

    // Empty the current theme configuration for Hyprland
    fs::write(&hypr_theme_file, "").expect("Failed to empty theme configuration");

    // reset any gtk theme configurations
    Command::new("gsettings")
        .args(["reset", "org.gnome.desktop.interface", "gtk-theme"])
        .status()
        .expect("failed to reset gtk-theme");

    // write selected theme name into the theme name log file
    // Ensure the directory exists
    fs::create_dir_all(&aurora_data).expect("Failed to create logs directory");

    // Now write the file
    let theme_name_log_file = aurora_data.join("theme_name.log");

    fs::write(&theme_name_log_file, theme_name)
        .expect("Failed to write theme name into the theme logs file.");
    // write theme logs
    let theme_log_file = aurora_data.join("theme.log");

    if let Some(config) = &config {
        let authors = config.authors.join(", ");
        let wallpaper_sources = match &config.wallpapers_sources {
            Some(sources) => sources.join(", "),
            None => "None".to_string(),
        };

        let information_about_theme = format!(
            "Theme Name = {}\nTheme Version = {}\nAuthors = {}\nRepo url = {}\nLicense = {}\nWallpaper sources = {}",
            config.name,
            config.version,
            authors,
            config.repo_url.as_deref().unwrap_or("None"),
            config.license.as_deref().unwrap_or("None"),
            wallpaper_sources
        );

        fs::write(&theme_log_file, information_about_theme).expect("Failed to write theme info");
    }

    println!("Applying theme : {}", theme_name);

    // Get the options from valid categories and do the job
    if let Some(config) = &config {
        if let Some(settings) = &config.settings {
            if let Some(script) = &settings.script {
                let interpreter = settings.interpreter.as_deref().unwrap_or("bash"); //get the parsed interpreter or simply set it to bash

                let mut path = paths.themes.clone();
                path.push(theme_name);
                path.push(script);

                Command::new(interpreter)
                    .arg(path)
                    .spawn()
                    .expect("Failed to run script"); //if it is failed
            } else {
                //skip if no script variable found in settings category file is available
                println!("No script in config, skipping...");
            }
        } else {
            //skip if no settings category exists in config file
            println!("No settings in config, skipping script...");
        }
    } else {
        //skip if no readable/valid configuration file is available
        println!("No valid config.toml found, skipping script...");
    }

    //Send the message for selected theme is applied
    Command::new("notify-send")
        .args([message])
        .output()
        .expect("failed to execute process");

    // copy the configured theme directories, including nested files like nvim/lua/plugins/*
    for folder in folders {
        let source = paths.themes.join(theme_name).join(folder);
        let target = paths.config.join(folder);

        if source.exists() {
            match copy_theme_path(&source, &target) {
                Ok(_) => println!("Applied {}", folder),
                Err(e) => eprintln!("Copy error in {}: {}", folder, e),
            }
        } else {
            println!("No theme directory found for {}", folder);
        }
    }

    // Run refresh script for refreshing the system
    Command::new("refresh_system")
        .spawn()
        .expect("Failed to refresh the system.");

    //get the default wallpaper
    let mut wallpaper = paths.themes.clone();
    wallpaper.push(theme_name);
    wallpaper.push("default.png");

    if wallpaper.exists() {
        println!("Setting wallpaper: {:?}", wallpaper);

        //use awww to apply the wallpaper
        Command::new("awww")
            .args([
                "img",
                wallpaper.to_str().unwrap(),
                "--transition-type",
                "grow",
                "--transition-duration",
                "1",
            ])
            .spawn()
            .ok();
    } else {
        println!("No default.jpg found for this theme");
    }
}

fn copy_theme_path(source: &Path, target: &Path) -> std::io::Result<()> {
    if source.is_file() {
        if should_copy(source, target)? {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(source, target)?;
        }
        return Ok(());
    }

    fs::create_dir_all(target)?;

    let mut pending = vec![(source.to_path_buf(), target.to_path_buf())];

    while let Some((current_source, current_target)) = pending.pop() {
        fs::create_dir_all(&current_target)?;

        for entry in fs::read_dir(current_source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_target = current_target.join(entry.file_name());

            if entry.file_type()?.is_dir() {
                pending.push((entry_path, entry_target));
                continue;
            }

            if should_copy(&entry_path, &entry_target)? {
                if let Some(parent) = entry_target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(entry_path, entry_target)?;
            }
        }
    }

    Ok(())
}

fn should_copy(source: &Path, target: &Path) -> std::io::Result<bool> {
    let source_meta = fs::metadata(source)?;

    let target_meta = match fs::metadata(target) {
        Ok(meta) => meta,
        Err(_) => return Ok(true),
    };

    if !target_meta.is_file() || source_meta.len() != target_meta.len() {
        return Ok(true);
    }

    let source_modified = source_meta.modified()?;
    let target_modified = target_meta.modified()?;

    if source_modified > target_modified {
        return Ok(true);
    }

    Ok(!files_match(source, target)?)
}

fn files_match(source: &Path, target: &Path) -> std::io::Result<bool> {
    const BUFFER_SIZE: usize = 16 * 1024;

    let mut source_file = fs::File::open(source)?;
    let mut target_file = fs::File::open(target)?;
    let mut source_buffer = [0_u8; BUFFER_SIZE];
    let mut target_buffer = [0_u8; BUFFER_SIZE];

    loop {
        let source_read = source_file.read(&mut source_buffer)?;
        let target_read = target_file.read(&mut target_buffer)?;

        if source_read != target_read {
            return Ok(false);
        }

        if source_read == 0 {
            return Ok(true);
        }

        if source_buffer[..source_read] != target_buffer[..target_read] {
            return Ok(false);
        }
    }
}

pub fn waybar_position_change(position: String) -> std::io::Result<()> {
    let paths = aurora_paths();
    let waybar_config = paths.config.join("waybar/config.jsonc");
    let content = fs::read_to_string(&waybar_config)?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Replace the Waybar position line regardless of the current value.
    for line in &mut lines {
        if line.trim_start().starts_with("\"position\":") {
            *line = format!("  \"position\": \"{}\",", position);
        }
    }

    let new_content = lines.join("\n");

    fs::write(waybar_config, new_content)?;

    println!("Updated!");

    Ok(())
}
// load the `style.css` file for gtk apps
pub fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_data(include_str!("./style.css"));

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        eprintln!("Warning: Could not connect to a display.");
    }
}

pub fn download_theme(repo_url: String) {
    let paths = aurora_paths();

    Command::new("git")
        .current_dir(paths.themes)
        .args(["clone", repo_url.as_str()])
        .status()
        .expect("Failed to clone theme");
}

#[cfg(test)]
mod tests {
    use super::{copy_theme_path, should_copy};
    use std::fs;
    use std::path::PathBuf;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn test_dir(name: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("aurora-{name}-{suffix}"))
    }

    #[test]
    fn should_copy_when_target_is_newer_but_contents_differ() {
        let root = test_dir("should-copy");
        fs::create_dir_all(&root).unwrap();

        let source = root.join("source.txt");
        let target = root.join("target.txt");

        fs::write(&source, "theme-a").unwrap();
        thread::sleep(Duration::from_millis(20));
        fs::write(&target, "theme-b").unwrap();

        assert!(should_copy(&source, &target).unwrap());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn copy_theme_path_copies_nested_files() {
        let root = test_dir("nested-copy");
        let source = root.join("source");
        let target = root.join("target");

        fs::create_dir_all(source.join("nvim/lua/plugins")).unwrap();
        fs::write(source.join("nvim/lua/plugins/init.lua"), "return {}").unwrap();

        copy_theme_path(&source, &target).unwrap();

        assert_eq!(
            fs::read_to_string(target.join("nvim/lua/plugins/init.lua")).unwrap(),
            "return {}"
        );

        fs::remove_dir_all(root).unwrap();
    }
}
