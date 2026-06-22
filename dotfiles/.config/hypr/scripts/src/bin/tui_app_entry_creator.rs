// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use aurora::{check_terminal, create_tui_desktop_entry, download_icon, load_css};
use gtk::gdk;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Entry,
    EventControllerKey, Label, Orientation,
};
use gtk4 as gtk;

fn validate_url(raw: &str) -> bool {
    let s = raw.trim();
    if s.is_empty() { return false; }
    let after = if let Some(rest) = s.strip_prefix("https://") { rest }
    else if let Some(rest) = s.strip_prefix("http://") { rest }
    else { return false; };
    if after.is_empty() || after.contains(' ') { return false; }
    after.contains('.') || after.eq_ignore_ascii_case("localhost")
}

fn validate_icon_url(raw: &str) -> bool {
    let s = raw.trim();
    s.is_empty() || (validate_url(s) && s.to_lowercase().ends_with(".png"))
}

fn validate_name(raw: &str) -> bool {
    raw.trim().len() >= 2
}

fn validate_command(raw: &str) -> bool {
    !raw.trim().is_empty()
}

fn set_entry_state(entry: &Entry, valid: bool) {
    entry.remove_css_class("valid");
    entry.remove_css_class("error");
    if valid { entry.add_css_class("valid"); }
    else { entry.add_css_class("error"); }
}

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.tui_app_entry_creator")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(520)
            .default_height(480)
            .title("Create TUI App")
            .decorated(false)
            .build();
        window.add_css_class("create-app-window");

        let main_box = GtkBox::new(Orientation::Vertical, 10);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);
        main_box.set_margin_start(24);
        main_box.set_margin_end(24);
        main_box.set_margin_top(24);
        main_box.set_margin_bottom(24);

        let header = Label::builder()
            .label("Create TUI App")
            .halign(Align::Center)
            .build();
        header.add_css_class("app-header");

        let subtitle = Label::builder()
            .label("Turn any terminal command into a desktop application")
            .halign(Align::Center)
            .wrap(true)
            .build();
        subtitle.add_css_class("setting-description");
        main_box.append(&header);
        main_box.append(&subtitle);

        let terminal_ok = check_terminal().is_ok();
        let terminal_warn = Label::builder()
            .label("")
            .halign(Align::Center)
            .wrap(true)
            .build();
        terminal_warn.add_css_class("chrome-warn");
        if !terminal_ok { terminal_warn.set_label("TERMINAL environment variable is not set"); }
        main_box.append(&terminal_warn);

        // App Name field
        let name_label = Label::builder()
            .label("App Name")
            .halign(Align::Start)
            .build();
        name_label.add_css_class("field-label");

        let name_entry = Entry::builder()
            .placeholder_text("e.g. btop, htop, nvim")
            .hexpand(true)
            .build();
        name_entry.add_css_class("app-entry");

        let name_indicator = Label::builder()
            .label("●")
            .halign(Align::Center)
            .valign(Align::Center)
            .width_chars(2)
            .build();
        name_indicator.add_css_class("status-indicator");

        let name_row = GtkBox::new(Orientation::Horizontal, 6);
        name_row.set_hexpand(true);
        name_row.append(&name_entry);
        name_row.append(&name_indicator);

        main_box.append(&name_label);
        main_box.append(&name_row);

        // Command field
        let command_label = Label::builder()
            .label("Command")
            .halign(Align::Start)
            .build();
        command_label.add_css_class("field-label");

        let command_entry = Entry::builder()
            .placeholder_text("e.g. btop, htop, /usr/bin/nvim")
            .hexpand(true)
            .build();
        command_entry.add_css_class("app-entry");

        let command_indicator = Label::builder()
            .label("●")
            .halign(Align::Center)
            .valign(Align::Center)
            .width_chars(2)
            .build();
        command_indicator.add_css_class("status-indicator");

        let command_row = GtkBox::new(Orientation::Horizontal, 6);
        command_row.set_hexpand(true);
        command_row.append(&command_entry);
        command_row.append(&command_indicator);

        main_box.append(&command_label);
        main_box.append(&command_row);

        // Icon URL field (optional)
        let icon_label = Label::builder()
            .label("Icon URL (optional)")
            .halign(Align::Start)
            .build();
        icon_label.add_css_class("field-label");

        let icon_entry = Entry::builder()
            .placeholder_text("e.g. https://example.com/icon.png (optional)")
            .hexpand(true)
            .build();
        icon_entry.add_css_class("app-entry");

        let icon_indicator = Label::builder()
            .label("●")
            .halign(Align::Center)
            .valign(Align::Center)
            .width_chars(2)
            .build();
        icon_indicator.add_css_class("status-indicator");

        let icon_row = GtkBox::new(Orientation::Horizontal, 6);
        icon_row.set_hexpand(true);
        icon_row.append(&icon_entry);
        icon_row.append(&icon_indicator);

        main_box.append(&icon_label);
        main_box.append(&icon_row);

        let sep = Label::builder().label("").build();
        main_box.append(&sep);

        let status_label = Label::builder()
            .label("Fill in all fields to continue")
            .halign(Align::Center)
            .wrap(true)
            .build();
        status_label.add_css_class("status-message");
        main_box.append(&status_label);

        let submit_btn = Button::builder()
            .label("Create TUI App")
            .halign(Align::Center)
            .sensitive(false)
            .build();
        submit_btn.add_css_class("submit-button");

        let btn_box = GtkBox::new(Orientation::Horizontal, 0);
        btn_box.set_halign(Align::Center);
        btn_box.set_margin_top(6);
        btn_box.append(&submit_btn);
        main_box.append(&btn_box);

        // Real-time validation
        let ne2 = name_entry.clone();
        let ce2 = command_entry.clone();
        let ie2 = icon_entry.clone();
        let sb2 = submit_btn.clone();
        let sl2 = status_label.clone();

        name_entry.connect_changed({
            let name_ind = name_indicator.clone();
            let ne = ne2.clone();
            let ce = ce2.clone();
            let ie = ie2.clone();
            let sb = sb2.clone();
            let sl = sl2.clone();
            let co = terminal_ok;
            move |e| {
                let v = validate_name(&e.text());
                set_entry_state(e, v);
                if e.text().trim().is_empty() {
                    name_ind.remove_css_class("valid");
                    name_ind.remove_css_class("error");
                } else if v {
                    name_ind.add_css_class("valid");
                    name_ind.remove_css_class("error");
                } else {
                    name_ind.remove_css_class("valid");
                    name_ind.add_css_class("error");
                }
                let nv = validate_name(&ne.text());
                let cv = validate_command(&ce.text());
                let iv = validate_icon_url(&ie.text());
                let all_ok = nv && cv && iv && co;
                sb.set_sensitive(all_ok);
                if all_ok {
                    sl.set_label("Ready to create!");
                    sl.remove_css_class("msg-error");
                    sl.add_css_class("msg-ready");
                } else {
                    sl.set_label("Fill in all fields correctly");
                    sl.remove_css_class("msg-ready");
                    sl.add_css_class("msg-error");
                }
            }
        });

        command_entry.connect_changed({
            let command_ind = command_indicator.clone();
            let ne = ne2.clone();
            let ce = ce2.clone();
            let ie = ie2.clone();
            let sb = sb2.clone();
            let sl = sl2.clone();
            let co = terminal_ok;
            move |e| {
                let v = validate_command(&e.text());
                set_entry_state(e, v);
                if e.text().trim().is_empty() {
                    command_ind.remove_css_class("valid");
                    command_ind.remove_css_class("error");
                } else if v {
                    command_ind.add_css_class("valid");
                    command_ind.remove_css_class("error");
                } else {
                    command_ind.remove_css_class("valid");
                    command_ind.add_css_class("error");
                }
                let nv = validate_name(&ne.text());
                let cv = validate_command(&ce.text());
                let iv = validate_icon_url(&ie.text());
                let all_ok = nv && cv && iv && co;
                sb.set_sensitive(all_ok);
                if all_ok {
                    sl.set_label("Ready to create!");
                    sl.remove_css_class("msg-error");
                    sl.add_css_class("msg-ready");
                } else {
                    sl.set_label("Fill in all fields correctly");
                    sl.remove_css_class("msg-ready");
                    sl.add_css_class("msg-error");
                }
            }
        });

        icon_entry.connect_changed({
            let icon_ind = icon_indicator.clone();
            let ne = ne2.clone();
            let ce = ce2.clone();
            let ie = ie2.clone();
            let sb = sb2.clone();
            let sl = sl2.clone();
            let co = terminal_ok;
            move |e| {
                let v = validate_icon_url(&e.text());
                set_entry_state(e, v);
                if e.text().trim().is_empty() {
                    icon_ind.remove_css_class("valid");
                    icon_ind.remove_css_class("error");
                } else if v {
                    icon_ind.add_css_class("valid");
                    icon_ind.remove_css_class("error");
                } else {
                    icon_ind.remove_css_class("valid");
                    icon_ind.add_css_class("error");
                }
                let nv = validate_name(&ne.text());
                let cv = validate_command(&ce.text());
                let iv = validate_icon_url(&ie.text());
                let all_ok = nv && cv && iv && co;
                sb.set_sensitive(all_ok);
                if all_ok {
                    sl.set_label("Ready to create!");
                    sl.remove_css_class("msg-error");
                    sl.add_css_class("msg-ready");
                } else {
                    sl.set_label("Fill in all fields correctly");
                    sl.remove_css_class("msg-ready");
                    sl.add_css_class("msg-error");
                }
            }
        });

        // Submit handler
        submit_btn.clone().connect_clicked(move |_| {
            let app_name = name_entry.text().trim().to_string();
            let app_command = command_entry.text().trim().to_string();
            let icon_url = icon_entry.text().trim().to_string();

            submit_btn.set_sensitive(false);

            let name_c = app_name.clone();
            let command_c = app_command.clone();
            let icon_c = icon_url.clone();
            let status_c = status_label.clone();
            let submit_c = submit_btn.clone();

            gtk::glib::MainContext::default().spawn_local(async move {
                let icon_path: String = if icon_c.is_empty() {
                    status_c.set_label("Creating desktop entry...");
                    String::new()
                } else {
                    status_c.set_label("Downloading icon...");
                    match download_icon(&icon_c, &name_c) {
                        Ok(p) => p.to_string_lossy().to_string(),
                        Err(e) => {
                            status_c.set_label(&e);
                            submit_c.set_sensitive(true);
                            return;
                        }
                    }
                };

                status_c.set_label("Creating desktop entry...");
                match create_tui_desktop_entry(&name_c, &command_c, &icon_path) {
                    Ok(()) => {
                        status_c.set_label(
                            &format!("Created '{}' successfully!", name_c),
                        );
                        status_c.remove_css_class("msg-error");
                        status_c.add_css_class("msg-ready");
                        submit_c.set_sensitive(true);
                    }
                    Err(e) => {
                        status_c.set_label(&e);
                        status_c.remove_css_class("msg-ready");
                        status_c.add_css_class("msg-error");
                        submit_c.set_sensitive(true);
                    }
                }
            });
        });

        window.set_child(Some(&main_box));

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
