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
use gtk::{Align, Application, ApplicationWindow, EventControllerKey, Label};
use gtk4 as gtk;
use whoami;

fn main() {
    let username = whoami::username().unwrap().to_string().to_uppercase();
    let app = Application::builder()
        .application_id("com.aurora.welcome")
        .build();

    app.connect_activate(move |app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(600)
            .default_height(400)
            .title("Welcome to Aurora")
            .decorated(false)
            .build();

        // Vertical box filling the window
        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 20);
        vbox.set_hexpand(true);
        vbox.set_vexpand(true);
        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);

        let wish = format!(
            "Hello <span weight='bold' foreground='violet'>{}</span>",
            username
        );
        let hello: Label = Label::builder()
            .label(&wish)
            .halign(Align::Center)
            .use_markup(true)
            .valign(Align::Center)
            .build();

        hello.add_css_class("hello");

        let welcome_label = Label::builder()
            .label("Welcome to <span weight='bold' foreground='purple'>Aurora</span>")
            .halign(Align::Center)
            .use_markup(true)
            .valign(Align::Center)
            .build();
        welcome_label.add_css_class("welcome-label"); // CSS class

        let keybind_label = Label::builder()
            .label(
                "Press <span foreground='grey'  weight='bold'>SUPER + H</span> to see all keybinds",
            )
            .halign(Align::Center)
            .valign(Align::Center)
            .use_markup(true)
            .build();
        keybind_label.add_css_class("keybind-label"); // CSS class

        vbox.append(&welcome_label);
        vbox.append(&keybind_label);
        vbox.append(&hello);

        let win = window.clone();

        window.set_child(Some(&vbox));

        let controller = EventControllerKey::new();

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
