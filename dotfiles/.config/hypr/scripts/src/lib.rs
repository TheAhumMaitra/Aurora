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

use std::path::PathBuf;
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

    themes.sort_by(|a, b| a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()));
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

    let folders = ["waybar", "wlogout", "hypr", "rofi"]; //directories want to be copied
    let filenames = ["colors.css", "colors.lua", "colors.rasi", "config.toml"]; //files need to be copied form that directories

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

    // copy the files
    for folder in folders {
        let mut found = false;

        for file in filenames {
            // source path is directory of the theme folder
            let mut source = paths.themes.clone();
            source.push(theme_name);
            source.push(folder);
            source.push(file);

            // our target is `.config`
            let mut target = paths.config.clone();
            target.push(folder);
            target.push(file);

            // if theme file exists
            if source.exists() {
                // if the parent directory exists (.config)
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).ok();
                }

                // copy the files from source to target
                match fs::copy(&source, &target) {
                    Ok(_) => println!("Applied {}/{}", folder, file),
                    Err(e) => eprintln!("Copy error in {}: {}", folder, e),
                }

                found = true;
            }
        }

        if !found {
            println!("No colors file found in {}", folder);
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
