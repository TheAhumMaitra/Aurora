// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use aurora::{load_css, load_inbox_entries, save_inbox_entry};
use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, DropDown, Entry,
    EventControllerKey, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, StringList,
};

fn refresh(list: &ListBox) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
    for entry in load_inbox_entries().into_iter().take(12) {
        let row = ListBoxRow::new();
        row.add_css_class("inbox-row");
        let box_ = GtkBox::new(Orientation::Vertical, 3);
        let kind = Label::builder()
            .label(entry.kind.to_uppercase())
            .halign(Align::Start)
            .build();
        kind.add_css_class("inbox-kind");
        let text = Label::builder()
            .label(&entry.text)
            .halign(Align::Start)
            .wrap(true)
            .build();
        text.add_css_class("inbox-text");
        box_.append(&kind);
        box_.append(&text);
        row.set_child(Some(&box_));
        list.append(&row);
    }
}

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.inbox")
        .build();
    app.connect_activate(|app| {
        load_css();
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Aurora Inbox")
            .default_width(520)
            .default_height(540)
            .decorated(false)
            .build();
        window.add_css_class("home-window");

        let title = Label::builder()
            .label("Inbox")
            .halign(Align::Center)
            .build();
        title.add_css_class("home-header");
        let subtitle = Label::builder()
            .label("Capture it now. Sort it out later.")
            .halign(Align::Center)
            .build();
        subtitle.add_css_class("home-subtitle");
        let input = Entry::builder()
            .placeholder_text("Capture an idea, task, or link...")
            .hexpand(true)
            .build();
        input.add_css_class("app-entry");
        let types = StringList::new(&["Idea", "Task", "Link", "Note"]);
        let kind = DropDown::new(Some(types), None::<gtk4::Expression>);
        kind.add_css_class("inbox-kind-select");
        let add = Button::with_label("Save to Inbox");
        add.add_css_class("home-create-btn");
        let list = ListBox::new();
        list.set_selection_mode(gtk4::SelectionMode::None);
        list.add_css_class("home-list");
        let scroll = ScrolledWindow::builder().vexpand(true).child(&list).build();

        let root = GtkBox::new(Orientation::Vertical, 12);
        root.set_margin_start(22);
        root.set_margin_end(22);
        root.set_margin_top(22);
        root.set_margin_bottom(22);
        root.append(&title);
        root.append(&subtitle);
        root.append(&input);
        root.append(&kind);
        root.append(&add);
        root.append(&scroll);
        window.set_child(Some(&root));
        refresh(&list);

        add.connect_clicked({
            let input = input.clone();
            let kind = kind.clone();
            let list = list.clone();
            move |_| {
                let text = input.text().trim().to_string();
                if text.is_empty() {
                    return;
                }
                let selected = kind.selected();
                let label = ["Idea", "Task", "Link", "Note"][selected as usize];
                match save_inbox_entry(&text, label) {
                    Ok(()) => {
                        input.set_text("");
                        refresh(&list);
                    }
                    Err(error) => eprintln!("Could not save inbox entry: {error}"),
                }
            }
        });
        input.connect_activate({
            let add = add.clone();
            move |_| add.emit_clicked()
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
        input.grab_focus();
    });
    app.run();
}
