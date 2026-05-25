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
use gtk::gdk;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Grid, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, SearchEntry,
};
use gtk4 as gtk;

struct KeybindCategory {
    title: &'static str,
    keybinds: &'static [(&'static str, &'static str)],
}
const KEYBIND_CATEGORIES: &[KeybindCategory] = &[
    KeybindCategory {
        title: "Hyprland keybinds",
        keybinds: &[
            ("Close current window", "SUPER + C"),
            ("Exit Hyprland", "SUPER + M"),
            ("Switch keyboard layout", "SUPER +SPACE"),
            ("Move focus to up window", "SUPER + UP"),
            ("Move focus to down window", "SUPER + DOWN"),
            ("Move focus to right window", "SUPER + RIGHT"),
            ("Move focus to left window", "SUPER + LEFT"),
            ("Toggle floating mode", "SUPER + V"),
            ("Toggle pseudo mode", "SUPER + P"),
            ("Open scratchpad workspace", "SUPER + S"),
            ("Move current window to scratchpad", "SUPER + SHIFT + S"),
            ("Resize window right", "SUPER + SHIFT + RIGHT"),
            ("Resize window left", "SUPER + SHIFT + LEFT"),
            ("Resize window up", "SUPER + SHIFT + UP"),
            ("Resize window down", "SUPER + SHIFT + DOWN"),
            ("Switch to dwindle layout", "SUPER + B"),
            ("Switch to master layout", "SUPER + K"),
            ("Switch to scrolling layout", "SUPER + X"),
            ("Switch to monocle layout", "SUPER + Z"),
            ("Toggle dwindle split", "SUPER + J"),
            ("Open application launcher", "SUPER + R"),
            ("Take screenshot", "SUPER + ALT + Z"),
        ],
    },
    KeybindCategory {
        title: "Application keybinds",
        keybinds: &[
            ("Open terminal", "SUPER + Q"),
            ("Open file manager", "SUPER + E"),
            ("Open browser", "SUPER + ALT + B"),
            ("Open code editor", "SUPER + ALT + V"),
            ("Open emoji picker", "SUPER + ALT + E"),
            ("Open clipboard manager", "SUPER + SHIFT + V"),
        ],
    },
    KeybindCategory {
        title: "Aurora guis and scripts",
        keybinds: &[
            ("Open keybinds help", "SUPER + H"),
            ("Open theme switcher", "SUPER + T"),
            ("Open Hyprland layout switcher", "SUPER + ALT + L"),
            ("Open settings", "SUPER + SHIFT + Z"),
            ("Open search popup", "SUPER + ALT + S"),
            ("Refresh waybar", "SUPER + W"),
            ("Toggle waybar", "SUPER + SHIFT + W"),
            ("Change the waybar position", "SUPER + ALT + W"),
            ("Open wallpaper switcher (theme)", "SUPER + SHIFT + T"),
            ("Open wallpaper switcher (global)", "SUPER + SHIFT + I"),
            ("Open power menu", "SUPER + ALT + P"),
        ],
    },
    KeybindCategory {
        title: "System keybinds",
        keybinds: &[
            ("Lock screen", "SUPER + L"),
            ("Increase volume", "XF86AudioRaiseVolume"),
            ("Decrease volume", "XF86AudioLowerVolume"),
            ("Mute audio", "XF86AudioMute"),
            ("Mute microphone", "XF86AudioMicMute"),
            ("Increase brightness", "XF86MonBrightnessUp"),
            ("Decrease brightness", "XF86MonBrightnessDown"),
            ("Next media track", "XF86AudioNext"),
            ("Play/Pause media", "XF86AudioPlay / XF86AudioPause"),
            ("Previous media track", "XF86AudioPrev"),
        ],
    },
    KeybindCategory {
        title: "Workspace keybinds",
        keybinds: &[
            ("Switch workspace", "SUPER + [1-0]"),
            ("Move window to workspace", "SUPER + SHIFT + [1-0]"),
            ("Next workspace", "SUPER + Mouse Scroll Down"),
            ("Previous workspace", "SUPER + Mouse Scroll Up"),
        ],
    },
    KeybindCategory {
        title: "Scrolling layout keybinds",
        keybinds: &[
            ("Move column left", "SUPER + COMMA"),
            ("Move column right", "SUPER + PERIOD"),
            ("Swap column left", "SUPER + SHIFT + COMMA"),
            ("Swap column right", "SUPER + SHIFT + PERIOD"),
            ("Resize column", "SUPER + ALT + COMMA / PERIOD"),
        ],
    },
    KeybindCategory {
        title: "Monocle layout keybinds",
        keybinds: &[
            ("Cycle next window", "SUPER + COMMA"),
            ("Cycle previous window", "SUPER + PERIOD"),
        ],
    },
];

fn append_keybinds(list_box: &ListBox, query: &str) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let query = query.trim().to_lowercase();

    for category in KEYBIND_CATEGORIES {
        let category_matches = category.title.to_lowercase().contains(&query);
        let matching_keybinds: Vec<_> = category
            .keybinds
            .iter()
            .filter(|(action, combo)| {
                query.is_empty()
                    || category_matches
                    || action.to_lowercase().contains(&query)
                    || combo.to_lowercase().contains(&query)
            })
            .collect();

        if matching_keybinds.is_empty() {
            continue;
        }

        let header_row = ListBoxRow::new();
        header_row.set_selectable(false);
        header_row.set_activatable(false);

        let header = Label::builder()
            .label(category.title)
            .halign(Align::Start)
            .build();
        header.add_css_class("header");
        header_row.set_child(Some(&header));
        list_box.append(&header_row);

        for (action, combo) in matching_keybinds {
            let row = ListBoxRow::new();
            row.set_selectable(false);
            row.set_activatable(false);

            let grid = Grid::new();
            grid.set_hexpand(true);
            grid.set_column_spacing(16);
            grid.set_column_homogeneous(true);
            grid.add_css_class("keybinds-grid");

            let action_label = Label::builder().label(*action).halign(Align::Start).build();
            action_label.add_css_class("actions");

            let combo_label = Label::builder().label(*combo).halign(Align::Start).build();
            combo_label.add_css_class("keybinds");

            grid.attach(&action_label, 0, 0, 1, 1);
            grid.attach(&combo_label, 1, 0, 1, 1);

            row.set_child(Some(&grid));
            list_box.append(&row);
        }
    }
}

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.keybinds_help")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(480)
            .title("Keybinds Help")
            .decorated(false)
            .resizable(true)
            .build();

        window.add_css_class("keybinds-window");
        window.set_opacity(0.8);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk::SelectionMode::None);
        append_keybinds(&list_box, "");

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // optional
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .hexpand(true)
            .build();

        scroll.set_child(Some(&list_box));

        let note = Label::builder()
            .label("Note : SUPER key means the Windows key in your keyboard")
            .halign(Align::End)
            .valign(Align::End)
            .build();
        note.add_css_class("note-warning");

        let search_entry = SearchEntry::builder()
            .placeholder_text("Search...")
            .hexpand(true)
            .build();
        search_entry.add_css_class("search-entry");

        let search_bar = GtkBox::new(Orientation::Vertical, 0);
        search_bar.add_css_class("search-bar");
        search_bar.append(&search_entry);

        search_entry.connect_search_changed({
            let list_box = list_box.clone();
            move |entry| append_keybinds(&list_box, &entry.text())
        });

        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(&scroll);
        vbox.append(&note);
        vbox.append(&search_bar);

        window.set_child(Some(&vbox));
        let controller = EventControllerKey::new();

        let win = window.clone();

        controller.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                // Close the window
                win.close();
                return true.into(); // event handled
            }
            false.into()
        });
        window.add_controller(controller);
        window.present();
    });

    app.run();
}
