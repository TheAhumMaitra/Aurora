// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use aurora::{load_css, list_web_apps, remove_web_app};
use gtk::gdk;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button,
    EventControllerKey, Label, ListBox, ListBoxRow, Orientation,
    ScrolledWindow,
};
use gtk4 as gtk;

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.web_app_entries_center")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(480)
            .default_height(600)
            .title("Aurora Web Apps")
            .decorated(false)
            .build();
        window.add_css_class("home-window");

        let root = GtkBox::new(Orientation::Vertical, 10);
        root.set_hexpand(true);
        root.set_vexpand(true);
        root.set_margin_start(20);
        root.set_margin_end(20);
        root.set_margin_top(20);
        root.set_margin_bottom(20);

        // ── Header ──────────────────────────────────────────
        let header = Label::builder()
            .label("Web Apps")
            .halign(Align::Center)
            .build();
        header.add_css_class("home-header");
        root.append(&header);

        let count_label = Label::builder()
            .label("")
            .halign(Align::Center)
            .build();
        count_label.add_css_class("home-count");
        root.append(&count_label);

        // ── App list ────────────────────────────────────────
        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk::SelectionMode::None);
        list_box.add_css_class("home-list");

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .vexpand(true)
            .hexpand(true)
            .build();
        scroll.set_child(Some(&list_box));
        root.append(&scroll);

        // Clone GTK refs before defining nested fn (so the fn owns them)
        let list_box_c = list_box.clone();
        let count_clone = count_label.clone();

        fn refresh_list(
            list_box: &ListBox,
            count_label: &Label,
        ) {
            // Clear rows
            while let Some(child) = list_box.first_child() {
                list_box.remove(&child);
            }

            let apps = list_web_apps();

            if apps.is_empty() {
                let empty = Label::builder()
                    .label("No web apps yet.\nCreate your first one!")
                    .halign(Align::Center)
                    .valign(Align::Center)
                    .wrap(true)
                    .build();
                empty.add_css_class("home-empty");
                let row = ListBoxRow::new();
                row.set_child(Some(&empty));
                list_box.append(&row);
                count_label.set_label("0 apps");
                return;
            }

            count_label.set_label(
                &format!("{} app{}", apps.len(), if apps.len() == 1 { "" } else { "s" })
            );

            for app_data in apps {
                let row_box = GtkBox::new(Orientation::Horizontal, 8);
                row_box.set_hexpand(true);
                row_box.set_margin_start(8);
                row_box.set_margin_end(8);
                row_box.set_margin_top(6);
                row_box.set_margin_bottom(6);

                let info_box = GtkBox::new(Orientation::Vertical, 2);
                info_box.set_hexpand(true);

                let name_lbl = Label::builder()
                    .label(&app_data.name)
                    .halign(Align::Start)
                    .build();
                name_lbl.add_css_class("home-app-name");

                let url_lbl = Label::builder()
                    .label(&app_data.url)
                    .halign(Align::Start)
                    .wrap(true)
                    .build();
                url_lbl.add_css_class("home-app-url");

                info_box.append(&name_lbl);
                info_box.append(&url_lbl);
                row_box.append(&info_box);

                // Remove button – clone the button before connecting
                let remove_btn = Button::builder()
                    .label("Remove")
                    .halign(Align::End)
                    .valign(Align::Center)
                    .build();
                remove_btn.add_css_class("home-remove-btn");

                let app_for_rm = app_data.clone();
                let lb_c = list_box.clone();
                let ct_c = count_label.clone();
                // Clone the remove_btn so we can use it in the closure
                let btn_for_rm = remove_btn.clone();
                btn_for_rm.connect_clicked(move |_| {
                    let a = app_for_rm.clone();
                    match remove_web_app(&a) {
                        Ok(()) => refresh_list(&lb_c, &ct_c),
                        Err(e) => eprintln!("Remove failed: {}", e),
                    }
                });

                row_box.append(&remove_btn);

                let row = ListBoxRow::new();
                row.add_css_class("home-row");
                row.set_child(Some(&row_box));
                list_box.append(&row);
            }
        }

        refresh_list(&list_box_c, &count_clone);

        // ── Create button ───────────────────────────────────
        let create_btn = Button::builder()
            .label("Create Web App")
            .halign(Align::Center)
            .build();
        create_btn.add_css_class("home-create-btn");
        root.append(&create_btn);

        // Launch web app entry creator
        let wc = window.clone();
        create_btn.connect_clicked(move |_| {
            let _ = std::process::Command::new("web_app_entry_creator")
                .spawn();
            wc.close();
        });

        window.set_child(Some(&root));

        // ── Escape to close ──────────────────────────────────
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
