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

use std::process::Command;

/// ─── Main menu: each entry is a clickable category ───────────────────────
/// Selecting one opens its submenu (or runs a command directly).
const ENTRIES: &[(&str, &[&str])] = &[
    ("  Power",               &["__sub_power__"]),
    ("  Utilities",           &["__sub_utilities__"]),
    ("  Settings",           &["settings"]),
    ("  System",              &["__sub_system__"]),
];

// ─── Submenu entries ─────────────────────────────────────────────────────

const POWER_ENTRIES: &[(&str, &[&str])] = &[
    ("  Lock",                &["hyprlock"]),
    ("  Logout",              &["hyprctl dispatch exit"]),
    ("  Suspend",             &["systemctl suspend"]),
    ("  Reboot",              &["systemctl reboot"]),
    ("  Shutdown",            &["systemctl poweroff"]),
];

const UTILITIES_ENTRIES: &[(&str, &[&str])] = &[
    ("  Theme Switcher",      &["theme_switcher"]),
    ("  Starship Switcher",   &["starship_switcher"]),
    ("  Rofi Flavour",        &["rofi_config_switcher"]),
    ("  Waybar Position",     &["waybar_position_switcher"]),
    ("  Layout Switcher",     &["layout_switcher"]),
    ("  Keybinds Help",       &["keybinds_help"]),
    ("  App Entries",         &["app_entries_home"]),
    ("  Search",              &["search"]),
];

const SYSTEM_ENTRIES: &[(&str, &[&str])] = &[
    ("  Refresh System",      &["refresh_system"]),
    ("  Screenshot",          &["hyprshot -m output"]),
    ("  Screen Recorder",     &["screenrecorder"]),
    ("  Wallpaper (Theme)",   &["waytrogen_line_change_for_theme", "waytrogen"]),
    ("  Wallpaper (Global)",  &["waytrogen_line_change_for_global_wallpapers", "waytrogen"]),
    ("  Toggle Waybar",       &["waybar_toggle"]),
];

fn main() {
    let selection = show_rofi_menu("System Menu", ENTRIES);

    if selection.is_empty() {
        return;
    }

    // Route to the correct submenu or run a command directly.
    match selection.as_str() {
        "  Power"     => show_submenu("Power Menu", POWER_ENTRIES),
        "  Utilities" => show_submenu("Utilities Menu", UTILITIES_ENTRIES),
        "  System"    => show_submenu("System Menu", SYSTEM_ENTRIES),
        // Settings and everything else run as commands
        s => {
            if let Some(action) = ENTRIES.iter().find(|(label, _)| *label == s) {
                run_action(action.1);
            }
        }
    }
}

/// Show a submenu via rofi and run whatever the user picks.
fn show_submenu(prompt: &str, entries: &[(&str, &[&str])]) {
    let selection = show_rofi_menu(prompt, entries);

    if let Some(action) = entries.iter().find(|(label, _)| *label == selection) {
        run_action(action.1);
    }
}

/// Pipe entries into rofi -dmenu and return the selected label.
fn show_rofi_menu<'a>(prompt: &str, entries: &[(&'a str, &[&str])]) -> String {
    let mut input = String::new();
    for (label, _) in entries {
        input.push_str(label);
        input.push('\n');
    }

    // rofi auto-loads ~/.config/rofi/config.rasi, so theme colours apply automatically.
    let output = Command::new("rofi")
        .args(["-dmenu", "-p", prompt, "-i"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(input.as_bytes());
            }
            child.wait_with_output()
        });

    match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(e) => {
            eprintln!("Failed to launch rofi: {e}");
            String::new()
        }
    }
}

/// Run the commands for an action sequentially.
fn run_action(cmds: &[&str]) {
    for cmd in cmds {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let output = Command::new(parts[0])
            .args(&parts[1..])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();

        if let Err(e) = output {
            eprintln!("Failed to execute `{cmd}`: {e}");
        }
    }
}
