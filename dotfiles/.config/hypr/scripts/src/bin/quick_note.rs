// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

// A persistent desktop scratchpad. Changes are saved locally as they are typed.

use aurora::aurora_paths;
use aurora::load_css;
use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Label,
    Orientation, PolicyType, ScrolledWindow, TextBuffer, TextView,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const APP_ID: &str = "com.aurora.quick_note";

fn note_path() -> PathBuf {
    aurora_paths()
        .home
        .join(".local/share/Aurora/quick-note.md")
}

fn load_note() -> String {
    fs::read_to_string(note_path()).unwrap_or_default()
}

fn save_note(buffer: &TextBuffer, status: &Label) {
    let start = buffer.start_iter();
    let end = buffer.end_iter();
    let text = buffer.text(&start, &end, false);
    let path = note_path();

    if let Some(parent) = path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            status.set_label(&format!("Could not create note directory: {error}"));
            return;
        }
    }

    match fs::write(&path, text.as_str()) {
        Ok(()) => {
            let words = text.split_whitespace().count();
            let suffix = if words == 1 { "word" } else { "words" };
            status.set_label(&format!("Saved · {words} {suffix}"));
        }
        Err(error) => status.set_label(&format!("Could not save note: {error}")),
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aurora Quick Note")
        .default_width(620)
        .default_height(480)
        .decorated(false)
        .resizable(true)
        .build();
    window.add_css_class("quick-note-window");

    let title = Label::builder()
        .label("Quick Note")
        .halign(Align::Start)
        .build();
    title.add_css_class("quick-note-title");
    let subtitle = Label::builder()
        .label("A private scratchpad that is saved automatically")
        .halign(Align::Start)
        .build();
    subtitle.add_css_class("quick-note-subtitle");

    let editor = TextView::new();
    editor.add_css_class("quick-note-editor");
    editor.set_wrap_mode(gtk4::WrapMode::WordChar);
    editor.set_monospace(false);
    editor.set_top_margin(12);
    editor.set_bottom_margin(12);
    editor.set_left_margin(14);
    editor.set_right_margin(14);
    editor.set_vexpand(true);
    let buffer = editor.buffer();
    buffer.set_text(&load_note());

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .child(&editor)
        .build();
    scroll.add_css_class("quick-note-scroll");

    let status = Label::builder().halign(Align::Start).build();
    status.add_css_class("quick-note-status");
    save_note(&buffer, &status);

    let open_folder = Button::builder().icon_name("folder-open-symbolic").build();
    open_folder.add_css_class("quick-note-folder");
    open_folder.set_tooltip_text(Some("Open the note folder"));
    open_folder.connect_clicked(move |_| {
        if let Some(parent) = note_path().parent() {
            let _ = Command::new("xdg-open").arg(parent).spawn();
        }
    });

    let header_copy = GtkBox::new(Orientation::Vertical, 2);
    header_copy.set_hexpand(true);
    header_copy.append(&title);
    header_copy.append(&subtitle);

    let header = GtkBox::new(Orientation::Horizontal, 12);
    header.add_css_class("quick-note-header");
    header.append(&header_copy);
    header.append(&open_folder);

    let footer = GtkBox::new(Orientation::Horizontal, 0);
    footer.add_css_class("quick-note-footer");
    footer.append(&status);

    let root = GtkBox::new(Orientation::Vertical, 0);
    root.add_css_class("quick-note-root");
    root.append(&header);
    root.append(&scroll);
    root.append(&footer);
    window.set_child(Some(&root));

    buffer.connect_changed({
        let status = status.clone();
        move |buffer| save_note(buffer, &status)
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
    editor.grab_focus();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });
    app.run();
}
