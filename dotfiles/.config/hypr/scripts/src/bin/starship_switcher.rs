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

use aurora::load_css;
use dirs::home_dir;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Label, ListBox, ListBoxRow,
    Orientation, ScrolledWindow, SearchEntry,
};

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Dict mapping config display names to their config file paths (relative to HOME).
const CONFIGS: &[(&str, &str)] = &[
    ("Default", ".config/starship/configs/default.toml"),
    ("Colorvoid", ".config/starship/configs/colorvoid.toml"),
    ("Jetpack", ".config/starship/configs/jetpack.toml"),
];

fn resolved_config_paths() -> Vec<(&'static str, PathBuf)> {
    let home = home_dir().expect("Could not get HOME directory");
    CONFIGS
        .iter()
        .map(|(name, path)| (*name, home.join(path)))
        .collect()
}

fn index_by_name() -> HashMap<&'static str, PathBuf> {
    resolved_config_paths().into_iter().collect()
}

fn apply_starship_config(name: &str, path: &PathBuf) {
    let home = home_dir().expect("Could not get HOME directory");
    let destination = home.join(".config").join("starship.toml");

    match fs::copy(path, destination) {
        Ok(_) => {
            println!("Starship config `{name}` applied");
            let _ = Command::new("notify-send")
                .arg(format!("{name} starship config is applied!"))
                .spawn();
        }
        Err(err) => {
            eprintln!("Failed to apply starship config `{name}`: {err}");
            let _ = Command::new("notify-send")
                .arg(format!("Failed to apply {name} starship config!"))
                .spawn();
        }
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Starship Switcher")
        .default_width(340)
        .default_height(500)
        .decorated(false)
        .build();

    window.set_opacity(0.8);
    load_css();

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::Single);

    let search_entry = SearchEntry::builder()
        .placeholder_text("Search...")
        .hexpand(true)
        .build();

    let search_bar = GtkBox::new(Orientation::Vertical, 0);
    search_bar.add_css_class("search-bar");
    search_bar.append(&search_entry);

    search_entry.add_css_class("search-entry");

    let search_controller = EventControllerKey::new();
    let win_ref = window.clone();

    search_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            win_ref.close();
            return true.into();
        }
        false.into()
    });

    search_entry.add_controller(search_controller);

    list_box.set_filter_func({
        let search_entry = search_entry.clone();
        move |row| {
            let query = search_entry.text().trim().to_lowercase();

            if query.is_empty() {
                return true;
            }

            row.child()
                .and_then(|child| child.downcast::<Label>().ok())
                .map(|label| label.text().to_lowercase().contains(&query))
                .unwrap_or(false)
        }
    });

    search_entry.connect_search_changed({
        let list_box = list_box.clone();
        move |_| list_box.invalidate_filter()
    });

    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&list_box)
        .build();

    let index = index_by_name();

    for (name, path) in resolved_config_paths() {
        let display = if path.exists() {
            name.to_string()
        } else {
            format!("{name} (missing)")
        };

        let row = ListBoxRow::new();
        row.add_css_class("section-row-theme");
        row.set_widget_name(name);

        let label = Label::new(Some(&display));
        label.set_xalign(0.0);

        row.set_child(Some(&label));
        list_box.append(&row);
    }

    list_box.connect_row_activated(move |_, row| {
        let name = row.widget_name();
        if let Some(path) = index.get(name.as_str()) {
            apply_starship_config(name.as_str(), path);
        }
    });

    let controller = EventControllerKey::new();
    let win = window.clone();

    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            win.close();
            return true.into();
        }
        false.into()
    });

    window.add_controller(controller);
    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.append(&search_bar);
    vbox.append(&scroll);

    window.set_child(Some(&vbox));
    window.show();
}

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.starship_switcher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

