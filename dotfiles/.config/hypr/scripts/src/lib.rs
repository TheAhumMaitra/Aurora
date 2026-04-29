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


use std::fs;
use std::path::PathBuf;
use std::process::Command;

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

pub fn apply_theme(theme_name: &str) {
    let (themes_dir, config_base) = get_paths();

    let folders = ["waybar", "wlogout", "hypr"];
    let filenames = ["colors.css", "colors.lua"];
    
    let message = format!("{theme_name} is applied!");
    println!("Applying theme : {}", theme_name);
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