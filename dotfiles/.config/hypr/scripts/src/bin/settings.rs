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
    Align, Application, ApplicationWindow, Box, Button, Label, Orientation, ScrolledWindow,
};
use std::process::Command;

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.settings")
        .build();

    app.connect_startup(|app| {
        build_ui(app);
        load_css()
    });

    app.run();
}

fn build_ui(app: &Application) {
    // Main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Settings")
        .default_width(800)
        .default_height(400)
        .build();

    window.set_opacity(0.8);

    let main_title = Label::builder()
        .label("Aurora Settings")
        .halign(Align::Start)
        .valign(Align::Center)
        .build();

    // Vertical container for all settings
    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    let home = std::env::var("HOME").unwrap();

    vbox.add_css_class("settings");

    add_setting(&vbox, "Edit main Hyprland Configuration", {
        let home = home.clone();
        move || {
            println!("Opening hyprland.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/hyprland.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit default monitor configuration", {
        let home = home.clone();
        move || {
            println!("Opening monitor.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/monitor.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit default plugins configuration", {
        let home = home.clone();
        move || {
            println!("Opening plugins.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/plugins.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit default autostart apps configuration", {
        let home = home.clone();
        move || {
            println!("Opening autostart.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/autostart.conf", home))
                .spawn()
                .unwrap();
        }
    });
    add_setting(&vbox, "Edit default keybinds", {
        let home = home.clone();
        move || {
            println!("Opening keybinds.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/keybinds.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit Aurora's look and feel configuration", {
        let home = home.clone();
        move || {
            println!("Opening look_and_feel.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/look_and_feel.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Change global theme", {
        move || {
            println!("Opening Theme Switcher");

            Command::new("sh")
                .arg("-c")
                .arg(format!("theme_switcher"))
                .spawn()
                .unwrap();
        }
    });
    // Scrollable window
    let scroll = ScrolledWindow::builder()
        .min_content_height(400)
        .child(&vbox)
        .build();
    window.set_child(Some(&main_title));
    window.set_child(Some(&scroll));
    window.show();
}

fn add_setting<F: Fn() + 'static>(parent: &Box, text: &str, action: F) {
    let row = Box::new(Orientation::Horizontal, 10);
    row.add_css_class("setting-row"); // 👈 row class

    let label = Label::new(Some(text));
    label.set_xalign(0.0);
    label.set_hexpand(true);
    label.add_css_class("setting-label"); // 👈 label class

    let button = Button::with_label("Run");
    button.add_css_class("setting-button"); // 👈 button class

    button.connect_clicked(move |_| {
        action();
    });

    row.append(&label);
    row.append(&button);

    parent.append(&row);
}
