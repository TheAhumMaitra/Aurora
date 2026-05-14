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

use std::process::Command;

const LAYOUTS: &[&str] = &["dwindle", "master", "scrolling", "monocle", "grid"];

fn run_layout_command(layout: &str) {
    let command = format!(
        r#"hl.config({{ general = {{ layout = "{}" }} }})"#,
        layout
    );

    if let Err(err) = Command::new("hyprctl").arg("eval").arg(command).spawn() {
        eprintln!("Failed to run layout command `{layout}`: {err}");
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Layout Switcher")
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

    for layout_name in LAYOUTS {
        let row = ListBoxRow::new();
        row.add_css_class("section-row-theme");
        let label = Label::new(Some(layout_name));
        label.set_xalign(0.0);

        row.set_child(Some(&label));
        list_box.append(&row);
    }

    list_box.connect_row_activated(move |_, row| {
        let label = row.child().unwrap().downcast::<Label>().unwrap();
        let layout_name = label.text();

        if LAYOUTS.contains(&layout_name.as_str()) {
            run_layout_command(&layout_name);
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
        .application_id("com.aurora.layout_switcher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
