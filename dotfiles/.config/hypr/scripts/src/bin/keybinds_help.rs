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


use gtk::ScrolledWindow;
use gtk::gdk;
use aurora::load_css;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, EventControllerKey, Grid, Label};
use gtk4 as gtk;

const KEYBINDS: &[(&str, &str)] = &[
    ("Open default terminal", "SUPER + Q"),
    ("Open current window", "SUPER + C"),
    ("Exit Hyprland", "SUPER + M"),
    ("Move focus to up window", "SUPER + UP"),
    ("Move focus to down window", "SUPER + DOWN"),
    ("Move focus to right window", "SUPER + RIGHT"),
    ("Move focus to left window", "SUPER + LEFT"),
    ("Open default file manager", "SUPER + E"),
    ("Toggle window to floating", "SUPER + V"),
    ("Open scratchpad on current workspace", "SUPER + S"),
    ("Move current window to scratchpad", "SUPER + SHIFT + S"),
    ("Open default browser", "SUPER + ALT + B"),
    ("Open Hyprsettings", "SUPER + SHIFT + H"),
    ("Refresh waybar", "SUPER + W"),
    ("Toggle waybar", "SUPER + SHIFT + W"),
    ("Open web search", "SUPER + ALT + S"),
    ("Open emoji picker", "SUPER + ALT + E"),
    ("Open clipboard manger", "SUPER + SHIFT + V"),
    ("Open keybinds help", "SUPER + H"),
    ("Switch to dwindle layout", "SUPER + B"),
    ("Switch to scrolling layout", "SUPER + X"),
    ("Switch to monocle layout", "SUPER + Z"),
    ("Switch to master layout", "SUPER + B"),
    ("Open wlogout / power screen", "SUPER + ALT + P"),
    ("Lock your screen (Hyprlock)", "SUPER + L"),
    ("Resize window by right", "SUPER + SHIFT + RIGHT"),
    ("Resize window by left", "SUPER + SHIFT + LEFT"),
];

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

        let grid = Grid::new();
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_column_spacing(16);
        grid.set_row_spacing(8);
        grid.set_margin_start(5);
        grid.set_margin_end(5);
        grid.set_margin_top(5);
        grid.set_margin_bottom(5);
        grid.set_column_homogeneous(true);
        grid.add_css_class("keybinds-grid");

        let last_row = KEYBINDS.len() as i32 + 1;

        let note = Label::builder()
            .label("Note : SUPER key means the Windows key in your keyboard")
            .halign(Align::End)
            .valign(Align::End)
            .build();
        note.add_css_class("note-warning");
        grid.attach(&note, 0, last_row, 2, 3);

        // Headers
        let header_action = Label::builder()
            .label("Action")
            .halign(Align::Start)
            .build();
        header_action.add_css_class("header"); // <-- add CSS after building

        let header_combo = Label::builder()
            .label("Key Combo")
            .halign(Align::Start)
            .build();
        header_combo.add_css_class("header"); // <-- same here

        grid.attach(&header_action, 0, 0, 1, 1);
        grid.attach(&header_combo, 1, 0, 1, 1);

        // Keybinds
        for (row, (action, combo)) in KEYBINDS.iter().enumerate() {
            let action_label = Label::builder()
                .label(*action) // dereference &str
                .halign(Align::Start)
                .build();
            action_label.add_css_class("actions");

            let combo_label = Label::builder()
                .label(*combo) // dereference &str
                .halign(Align::Start)
                .build();
            combo_label.add_css_class("keybinds");

            grid.attach(&action_label, 0, (row + 1) as i32, 1, 1);
            grid.attach(&combo_label, 1, (row + 1) as i32, 1, 1);
        }

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // optional
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .build();

        scroll.set_child(Some(&grid));
        window.set_child(Some(&scroll));
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

