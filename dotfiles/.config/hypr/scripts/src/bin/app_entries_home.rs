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
    Align, Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Label,
    Orientation,
};
use gtk4 as gtk;

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.app_entries_home")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Aurora Launchers")
            .default_width(420)
            .default_height(320)
            .decorated(false)
            .build();

        window.add_css_class("home-window");

        let root = GtkBox::new(Orientation::Vertical, 20);
        root.set_hexpand(true);
        root.set_vexpand(true);
        root.set_margin_start(36);
        root.set_margin_end(36);
        root.set_margin_top(36);
        root.set_margin_bottom(36);

        let header_box = GtkBox::new(Orientation::Vertical, 8);
        header_box.set_halign(Align::Center);

        let title = Label::builder()
            .label("App Entries Manager")
            .halign(Align::Center)
            .build();
        title.add_css_class("home-title");
        header_box.append(&title);

        let desc = Label::builder()
            .label("Launch app entries centers")
            .halign(Align::Center)
            .wrap(true)
            .max_width_chars(40)
            .build();
        desc.add_css_class("home-subtitle");
        header_box.append(&desc);

        root.append(&header_box);

        let sep = Label::builder().label("").build();
        root.append(&sep);

        let btn_box = GtkBox::new(Orientation::Vertical, 12);
        btn_box.set_halign(Align::Center);
        btn_box.set_hexpand(true);

        let web_btn = gtk::Button::builder()
            .label("Web Apps")
            .halign(Align::Fill)
            .hexpand(true)
            .build();
        web_btn.add_css_class("home-action-btn");
        btn_box.append(&web_btn);

        let win_c = window.clone();
        web_btn.connect_clicked(move |_| {
            let _ = std::process::Command::new("web_app_entries_center")
                .spawn();
            win_c.close();
        });

        let tui_btn = gtk::Button::builder()
            .label("Terminal Apps")
            .halign(Align::Fill)
            .hexpand(true)
            .build();
        tui_btn.add_css_class("home-action-btn");
        btn_box.append(&tui_btn);

        let win_c2 = window.clone();
        tui_btn.connect_clicked(move |_| {
            let _ = std::process::Command::new("tui_app_entries_center")
                .spawn();
            win_c2.close();
        });

        root.append(&btn_box);

        window.set_child(Some(&root));

        let ctrl = EventControllerKey::new();
        let win_close = window.clone();
        ctrl.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                win_close.close();
                return true.into();
            }
            false.into()
        });
        window.add_controller(ctrl);

        window.present();
    });

    app.run();
}
