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
use gtk::{Align, Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Label, Orientation};
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

        // Welcome label with separate Aurora text
        let welcome_box = GtkBox::new(Orientation::Horizontal, 0);
        welcome_box.set_halign(Align::Center);
        
        let welcome_text = Label::builder()
            .label("Welcome to ")
            .halign(Align::Center)
            .build();
        welcome_text.add_css_class("welcome-label");
        
        let aurora_text = Label::builder()
            .label("Aurora")
            .halign(Align::Center)
            .build();
        aurora_text.add_css_class("aurora-text");
        
        welcome_box.append(&welcome_text);
        welcome_box.append(&aurora_text);

        // Hello username with separate colored username
        let hello_box = GtkBox::new(Orientation::Horizontal, 0);
        hello_box.set_halign(Align::Center);
        
        let hello_text = Label::builder()
            .label("Hello ")
            .halign(Align::Center)
            .build();
        hello_text.add_css_class("hello");
        
        let username_text = Label::builder()
            .label(&username)
            .halign(Align::Center)
            .build();
        username_text.add_css_class("username");
        
        hello_box.append(&hello_text);
        hello_box.append(&username_text);

        let keybind_box = GtkBox::new(Orientation::Horizontal, 0);
        keybind_box.set_halign(Align::Center);
        
        let keybind_prefix = Label::builder()
            .label("Press ")
            .halign(Align::Center)
            .build();
        keybind_prefix.add_css_class("keybind-label");
        
        let keybind_key = Label::builder()
            .label("SUPER + H")
            .halign(Align::Center)
            .build();
        keybind_key.add_css_class("keybinds-help-warn");
        
        let keybind_suffix = Label::builder()
            .label(" to see all keybinds")
            .halign(Align::Center)
            .build();
        keybind_suffix.add_css_class("keybind-label");
        
        keybind_box.append(&keybind_prefix);
        keybind_box.append(&keybind_key);
        keybind_box.append(&keybind_suffix);

        vbox.append(&welcome_box);
        vbox.append(&keybind_box);
        vbox.append(&hello_box);

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
