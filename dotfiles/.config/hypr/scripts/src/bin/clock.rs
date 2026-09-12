// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

// A small clock app for an Aurora Hyprland desktop session.
// Shows the time in a large accent clock and the full date (with weekday)
// pinned to the bottom, styled with the same Aurora palette as the other apps.

use aurora::load_css;
use gtk4::gdk::Key;
use gtk4::glib::{ControlFlow, DateTime};
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, EventControllerKey, Label, Orientation,
};

const APP_ID: &str = "com.aurora.clock";

fn current_time() -> (String, String) {
    if let Ok(now) = DateTime::now_local() {
        (
            now.format("%H:%M:%S").unwrap_or_default().to_string(),
            now.format("%A, %d %B %Y").unwrap_or_default().to_string(),
        )
    } else {
        ("00:00:00".into(), "—".into())
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(430)
        .default_height(260)
        .decorated(false)
        .resizable(false)
        .build();
    window.add_css_class("clock-window");

    let quote = Label::builder()
        .label("Redeeming the time, because the days are evil")
        .halign(Align::Center)
        .build();
    quote.add_css_class("focus-subtitle");

    let clock = Label::new(None);
    clock.add_css_class("clock-display");
    clock.set_halign(Align::Center);

    let date = Label::new(None);
    date.add_css_class("clock-date");
    date.set_halign(Align::Center);

    let header = GtkBox::new(Orientation::Vertical, 3);
    header.append(&quote);

    // The surface that holds the big clock, pushed up by an expanding spacer
    // so the date always sits pinned at the bottom.
    let surface = GtkBox::new(Orientation::Vertical, 8);
    surface.add_css_class("clock-surface");
    surface.append(&clock);

    let root = GtkBox::new(Orientation::Vertical, 14);
    root.add_css_class("clock-root");
    root.append(&header);

    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);

    root.append(&spacer);
    root.append(&surface);
    root.append(&date);
    window.set_child(Some(&root));

    let (time, date_text) = current_time();
    clock.set_label(&time);
    date.set_label(&date_text);

    gtk4::glib::timeout_add_seconds_local(1, move || {
        let (time, date_text) = current_time();
        clock.set_label(&time);
        date.set_label(&date_text);
        ControlFlow::Continue
    });

    let controller = EventControllerKey::new();
    controller.connect_key_pressed({
        let window = window.clone();
        move |_, key, _, _| {
            if key == Key::Escape {
                window.close();
                return true.into();
            }
            false.into()
        }
    });
    window.add_controller(controller);
    window.present();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });
    app.run();
}
