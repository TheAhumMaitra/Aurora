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

fn get_paths() -> (PathBuf, PathBuf) {
    let home = std::env::var("HOME").expect("Could not get HOME");
    let themes_dir = PathBuf::from(format!("{}/.config/themes", home));
    let config_base = PathBuf::from(format!("{}/.config", home));
    (themes_dir, config_base)
}

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

#[derive(Deserialize, Debug)]
struct Config {
    settings: Settings,
}

#[derive(Deserialize, Debug)]
struct Settings {
    script: Option<String>,
    interpreter: Option<String>,
}
pub fn apply_theme(theme_name: &str) {
    let home = std::env::var("HOME").expect("Could not get HOME");
    let (themes_dir, config_base) = get_paths();
    let mut config_path = themes_dir.clone();
    config_path.push(theme_name);
    config_path.push("config.toml");

    let config: Option<Config> = fs::read_to_string(&config_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok());
    let folders = ["waybar", "wlogout", "hypr", "rofi"];
    let filenames = ["colors.css", "colors.lua", "colors.rasi", "config.toml"];

    let message = format!("{theme_name} is applied!");
    let logs_directory = format!("{}/.local/share/Aurora", home);
    let theme_log_path = format!("{}/theme_name.log", logs_directory);

    println!("Applying theme : {}", theme_name);

    if let Some(config) = config {
        if let Some(script) = &config.settings.script {
            let interpreter = config.settings.interpreter.as_deref().unwrap_or("bash");

            let mut path = themes_dir.clone();
            path.push(theme_name);
            path.push(script);

            Command::new(interpreter)
                .arg(path)
                .spawn()
                .expect("Failed to run script");
        } else {
            println!("No script in config, skipping...");
        }
    } else {
        println!("No config.toml found, skipping script...");
    }

    Command::new("notify-send")
        .args([message])
        .output()
        .expect("failed to execute process");

    for folder in folders {
        let mut found = false;

        for file in filenames {
            let mut source = themes_dir.clone();
            source.push(theme_name);
            source.push(folder);
            source.push(file);

            let mut target = config_base.clone();
            target.push(folder);
            target.push(file);

            if source.exists() {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).ok();
                }

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
    let exe = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".config/hypr/scripts/target/release/refresh_system");

    Command::new(exe)
        .spawn()
        .expect("failed to run refresh_system");

    let mut wallpaper = themes_dir.clone();
    wallpaper.push(theme_name);
    wallpaper.push("default.png");

    if wallpaper.exists() {
        println!("Setting wallpaper: {:?}", wallpaper);

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
pub fn load_css() {
    let provider = CssProvider::new();

    // include as &str, not bytes
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
