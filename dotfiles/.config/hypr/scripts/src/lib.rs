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

// Returns commonly used paths for theme switcher
fn get_paths() -> (PathBuf, PathBuf) {
    let home = std::env::var("HOME").expect("Could not get HOME");
    let themes_dir = PathBuf::from(format!("{}/.config/themes", home));
    let config_base = PathBuf::from(format!("{}/.config", home));
    (themes_dir, config_base)
}

// list all available themes in `~/.config/themes` for mostly cli
pub fn list_themes() {
    let (themes_dir, _) = get_paths();

    println!("Showing Available Themes:\n");

    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name: std::borrow::Cow<'_, str> = path.file_name().unwrap().to_string_lossy();
                println!("• {}", name);
            }
        }
    }
}

// Root structure for parsing config.toml
#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    version: String,
    authors: Vec<String>,
    repo_url: Option<String>,
    wallpapers_source: Option<String>,
    settings: Settings,
}

// get all the authors name and github
#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    github: String,
}

// get options inside of the toml file (we only need required options)
#[derive(Deserialize, Debug)]
struct Settings {
    script: Option<String>,
    interpreter: Option<String>,
}

// apply the selected theme
pub fn apply_theme(theme_name: &str) {
    let home = std::env::var("HOME").expect("Could not get HOME");
    let (themes_dir, config_base) = get_paths();

    // get the `config.toml` from the selected theme
    let mut config_path = themes_dir.clone();
    config_path.push(theme_name);
    config_path.push("config.toml");

    let config: Option<Config> = fs::read_to_string(&config_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok());

    let folders = ["waybar", "wlogout", "hypr", "rofi"]; //directories want to be copied
    let filenames = ["colors.css", "colors.lua", "colors.rasi", "config.toml"]; //files need to be copied form that directories

    let message = format!("{theme_name} is applied!");

    // Empty the current theme configuration for Hyprland
    // get theme configuration directory
    let theme_config = format!("{}/.config/hypr/Theme/theme.lua", home);
    fs::write(&theme_config, "").expect("Failed to empty theme configuration");

    // reset any gtk configuration
    Command::new("dconf")
        .args(["reset", "-f", "/org/gnome/"])
        .output()
        .expect("failed to reset gtk settings");

    //get the logs directory of Aurora
    let logs_directory = format!("{}/.local/share/Aurora", home);

    // write selected theme name into the theme name log file
    // Ensure the directory exists
    fs::create_dir_all(&logs_directory).expect("Failed to create logs directory");

    // Now write the file
    let theme_name_log_file = format!("{}/theme_name.log", logs_directory);

    fs::write(&theme_name_log_file, theme_name)
        .expect("Failed to write theme name into the theme logs file.");
    // write theme logs 
    let theme_log_file = format!("{}/theme.log", logs_directory);

    if let Some(config) = &config {
       let authors = config.authors.join(", ");

        let information_about_theme = format!(
            "Theme Name = {}\nTheme Version = {}\nAuthors = {}\nRepo url = {}\nWallpaper source = {}",
            config.name,
            config.version,
            authors,
            config.repo_url.as_deref().unwrap_or("None"),
            config.wallpapers_source.as_deref().unwrap_or("None")
        );

        fs::write(&theme_log_file, information_about_theme).expect("Failed to write theme info");
    }

    println!("Applying theme : {}", theme_name);

    // Get the options from valid categories and do the job
    if let Some(config) = &config {
        if let Some(script) = &config.settings.script {
            let interpreter = config.settings.interpreter.as_deref().unwrap_or("bash"); //get the parsed interpreter or simply set it to bash

            let mut path = themes_dir.clone();
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
        //skip if no configuration file is available
        println!("No config.toml found, skipping script...");
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
            let mut source = themes_dir.clone();
            source.push(theme_name);
            source.push(folder);
            source.push(file);

            // our target is `.config`
            let mut target = config_base.clone();
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
    let mut wallpaper = themes_dir.clone();
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
    Command::new("git")
        .current_dir(format!("{}/.config/themes", std::env::var("HOME").unwrap()))
        .args(["clone", repo_url.as_str()])
        .status()
        .expect("Failed to clone theme");
}