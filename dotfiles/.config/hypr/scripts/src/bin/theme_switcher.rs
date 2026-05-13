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
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Label, ListBox, ListBoxRow,
    Orientation, ScrolledWindow, SearchEntry,
};

use aurora::apply_theme;
use std::fs;
use std::path::PathBuf;

fn get_paths() -> (PathBuf, PathBuf) {
    let home = std::env::var("HOME").expect("Could not get HOME");
    let themes_dir = PathBuf::from(format!("{}/.config/themes", home));
    let config_base = PathBuf::from(format!("{}/.config", home));
    (themes_dir, config_base)
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Theme Switcher")
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

    let (themes_dir, _) = get_paths();

    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let theme_name = path.file_name().unwrap().to_string_lossy().to_string();

                let row = ListBoxRow::new();
                row.add_css_class("section-row-theme");
                let label = Label::new(Some(&theme_name));
                label.set_xalign(0.0);

                row.set_child(Some(&label));
                list_box.append(&row);
            }
        }
    }

    list_box.connect_row_activated(move |_, row| {
        let label = row.child().unwrap().downcast::<Label>().unwrap();
        let theme_name = label.text();

        apply_theme(&theme_name);
        load_css();
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

// main
fn main() {
    let app = Application::builder()
        .application_id("com.aurora.theme_switcher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
