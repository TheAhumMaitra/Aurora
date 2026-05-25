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

fn main() -> std::io::Result<()> {
    let home = std::env::var("HOME").unwrap();

    let path = format!("{}/.config/waytrogen/config.json", home);

    let content = fs::read_to_string(&path)?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Find wallpaper_folder line automatically
    for line in &mut lines {
        if line.contains("\"wallpaper_folder\"") {
            *line = format!("  \"wallpaper_folder\": \"~/Pictures/Wallpapers/\",",);
        }
    }

    let new_content = lines.join("\n");

    fs::write(path, new_content)?;

    println!("Updated!");

    Ok(())
}
