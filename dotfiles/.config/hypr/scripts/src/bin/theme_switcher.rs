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

use aurora::apply_theme;
use aurora::load_css;
use aurora::theme_entries;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Label, ListBox, ListBoxRow,
    Orientation, Picture, ScrolledWindow, SearchEntry,
};
use std::collections::HashMap;

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Theme Switcher")
        .default_width(960)
        .default_height(520)
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

    let preview_name = Label::new(None);
    preview_name.add_css_class("theme-preview-title");
    preview_name.set_xalign(0.0);

    let preview_picture = Picture::new();
    preview_picture.add_css_class("theme-preview-image");
    preview_picture.set_alternative_text(Some("Theme preview"));
    preview_picture.set_can_shrink(true);
    preview_picture.set_keep_aspect_ratio(false);
    preview_picture.set_hexpand(true);
    preview_picture.set_vexpand(true);

    let themes = theme_entries();
    let preview_paths: HashMap<_, _> = themes
        .iter()
        .map(|theme| (theme.directory_name.clone(), theme.preview_path.clone()))
        .collect();

    for theme in &themes {
        let row = ListBoxRow::new();
        row.add_css_class("section-row-theme");
        row.set_widget_name(&theme.directory_name);

        let label = Label::new(Some(&theme.display_name));
        label.set_xalign(0.0);

        row.set_child(Some(&label));
        list_box.append(&row);
    }

    list_box.connect_row_selected({
        let preview_name = preview_name.clone();
        let preview_picture = preview_picture.clone();
        let preview_paths = preview_paths.clone();

        move |_, row| {
            let Some(row) = row else {
                return;
            };

            let Some(label) = row.child().and_then(|child| child.downcast::<Label>().ok()) else {
                return;
            };

            preview_name.set_text(&label.text());

            if let Some(preview_path) = preview_paths.get(&row.widget_name().to_string()) {
                preview_picture.set_filename(Some(preview_path));
            }
        }
    });

    list_box.connect_row_activated(move |_, row| {
        apply_theme(&row.widget_name());
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
    let list_panel = GtkBox::new(Orientation::Vertical, 0);
    list_panel.add_css_class("theme-list-panel");
    list_panel.set_size_request(280, -1);
    list_panel.set_vexpand(true);
    list_panel.append(&search_bar);
    list_panel.append(&scroll);

    let preview_panel = GtkBox::new(Orientation::Vertical, 0);
    preview_panel.add_css_class("theme-preview-panel");
    preview_panel.set_hexpand(true);
    preview_panel.set_vexpand(true);
    preview_panel.append(&preview_name);
    preview_panel.append(&preview_picture);

    let hbox = GtkBox::new(Orientation::Horizontal, 0);
    hbox.add_css_class("theme-switcher-content");
    hbox.append(&list_panel);
    hbox.append(&preview_panel);

    if let Some(first_row) = list_box.row_at_index(0) {
        list_box.select_row(Some(&first_row));
    }

    window.set_child(Some(&hbox));
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
