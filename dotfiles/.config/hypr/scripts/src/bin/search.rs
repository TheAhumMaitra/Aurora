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


use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry};
use gtk4 as gtk;
use std::process::Command;

const SEARCH_ENGINE_NAME: &str = "Google";
const SEARCH_ENGINE_URL: &str = "https://www.google.com/search?q=";

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.search")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Search")
        .decorated(false)
        // .resizable(false) // important
        .build();

    window.set_opacity(0.85);

    let controller = gtk::EventControllerKey::new();

    let app_clone2 = app.clone();

    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            app_clone2.quit();
            return gtk::glib::Propagation::Stop;
        }
        gtk::glib::Propagation::Proceed
    });

    window.add_controller(controller);

    let entry = Entry::builder()
        .placeholder_text(&format!("Search {}...", SEARCH_ENGINE_NAME))
        .build();

    entry.set_size_request(400, 50);
    let app_clone = app.clone();
    entry.connect_activate(move |entry| {
        let text = entry.text().to_string();
        if text.trim().is_empty() {
            return;
        }

        let encoded = urlencoding::encode(&text);
        let url = format!("{}{}", SEARCH_ENGINE_URL, encoded);

        Command::new("xdg-open")
            .arg(url)
            .spawn()
            .expect("Failed to open browser");

        app_clone.quit();

        std::process::exit(0);
    });

    window.set_child(Some(&entry));
    window.present();
}
