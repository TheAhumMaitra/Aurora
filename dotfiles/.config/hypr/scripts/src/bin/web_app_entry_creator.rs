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

use aurora::{check_chrome, create_desktop_entry, download_icon, load_css};
use gtk::gdk;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Entry,
    EventControllerKey, Label, Orientation,
};
use gtk4 as gtk;

// ── Validation helpers ──────────────────────────────────────────────────────

/// Validate a generic URL (must have http/https scheme and a valid host).
fn validate_url(raw: &str) -> bool {
    let s = raw.trim();
    if s.is_empty() { return false; }
    let after = if let Some(rest) = s.strip_prefix("https://") { rest }
    else if let Some(rest) = s.strip_prefix("http://") { rest }
    else { return false; };
    if after.is_empty() || after.contains(' ') { return false; }
    after.contains('.') || after.eq_ignore_ascii_case("localhost")
}

/// Validate an icon URL – same as a normal URL but must end with .png.
/// An empty string is also valid (icon is optional).
fn validate_icon_url(raw: &str) -> bool {
    let s = raw.trim();
    s.is_empty() || (validate_url(s) && s.to_lowercase().ends_with(".png"))
}

/// Validate the app name – non-empty and at least 2 characters.
fn validate_name(raw: &str) -> bool {
    raw.trim().len() >= 2
}

/// Apply CSS classes to an entry based on validity.
fn set_entry_state(entry: &Entry, valid: bool) {
    entry.remove_css_class("valid");
    entry.remove_css_class("error");
    if valid { entry.add_css_class("valid"); }
    else { entry.add_css_class("error"); }
}


fn main() {
    let app = Application::builder()
        .application_id("com.aurora.create_app_entry")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(520)
            .default_height(480)
            .title("Create Web App")
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
            .label("Create Web App")
            .halign(Align::Center)
            .build();
        header.add_css_class("app-header");

        let subtitle = Label::builder()
            .label("Turn any website into a desktop application")
            .halign(Align::Center)
            .wrap(true)
            .build();
        subtitle.add_css_class("setting-description");
        main_box.append(&header);
        main_box.append(&subtitle);

        let chrome_ok = check_chrome().is_ok();
        let chrome_warn = Label::builder()
            .label("")
            .halign(Align::Center)
            .wrap(true)
            .build();
        chrome_warn.add_css_class("chrome-warn");
        if !chrome_ok { chrome_warn.set_label("Google Chrome is not installed"); }
        main_box.append(&chrome_warn);


        // ── App Name field ──────────────────────────────────
        let name_label = Label::builder()
            .label("App Name")
            .halign(Align::Start)
            .build();
        name_label.add_css_class("field-label");

        let name_entry = Entry::builder()
            .placeholder_text("e.g. YouTube, Notion, Figma")
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

        // ── App URL field ───────────────────────────────────
        let url_label = Label::builder()
            .label("App URL")
            .halign(Align::Start)
            .build();
        url_label.add_css_class("field-label");

        let url_entry = Entry::builder()
            .placeholder_text("e.g. https://youtube.com")
            .hexpand(true)
            .build();
        url_entry.add_css_class("app-entry");

        let url_indicator = Label::builder()
            .label("●")
            .halign(Align::Center)
            .valign(Align::Center)
            .width_chars(2)
            .build();
        url_indicator.add_css_class("status-indicator");

        let url_row = GtkBox::new(Orientation::Horizontal, 6);
        url_row.set_hexpand(true);
        url_row.append(&url_entry);
        url_row.append(&url_indicator);

        main_box.append(&url_label);
        main_box.append(&url_row);

        // ── Icon URL field (optional) ──────────────────────────
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

        // ── Separator ───────────────────────────────────────
        let sep = Label::builder().label("").build();
        main_box.append(&sep);

        // ── Status message ───────────────────────────────────
        let status_label = Label::builder()
            .label("Fill in all fields to continue")
            .halign(Align::Center)
            .wrap(true)
            .build();
        status_label.add_css_class("status-message");
        main_box.append(&status_label);

        // ── Submit button ────────────────────────────────────
        let submit_btn = Button::builder()
            .label("Create Web App")
            .halign(Align::Center)
            .sensitive(false)
            .build();
        submit_btn.add_css_class("submit-button");

        let btn_box = GtkBox::new(Orientation::Horizontal, 0);
        btn_box.set_halign(Align::Center);
        btn_box.set_margin_top(6);
        btn_box.append(&submit_btn);
        main_box.append(&btn_box);

        // ── Real-time validation ────────────────────────────
        // Shared clones for cross-field validation
        let ne2 = name_entry.clone();
        let ue2 = url_entry.clone();
        let ie2 = icon_entry.clone();
        let sb2 = submit_btn.clone();
        let sl2 = status_label.clone();

        // ── Name changed ────────────────────────────────────
        name_entry.connect_changed({
            let name_ind = name_indicator.clone();
            let ne = ne2.clone();
            let ue = ue2.clone();
            let ie = ie2.clone();
            let sb = sb2.clone();
            let sl = sl2.clone();
            let co = chrome_ok;
            move |e| {
                let v = validate_name(&e.text());
                set_entry_state(e, v);
                // Update indicator
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
                // Check all fields
                let nv = validate_name(&ne.text());
                let uv = validate_url(&ue.text());
                let iv = validate_icon_url(&ie.text());
                let all_ok = nv && uv && iv && co;
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

        // ── URL changed ─────────────────────────────────────
        url_entry.connect_changed({
            let url_ind = url_indicator.clone();
            let ne = ne2.clone();
            let ue = ue2.clone();
            let ie = ie2.clone();
            let sb = sb2.clone();
            let sl = sl2.clone();
            let co = chrome_ok;
            move |e| {
                let v = validate_url(&e.text());
                set_entry_state(e, v);
                if e.text().trim().is_empty() {
                    url_ind.remove_css_class("valid");
                    url_ind.remove_css_class("error");
                } else if v {
                    url_ind.add_css_class("valid");
                    url_ind.remove_css_class("error");
                } else {
                    url_ind.remove_css_class("valid");
                    url_ind.add_css_class("error");
                }
                let nv = validate_name(&ne.text());
                let uv = validate_url(&ue.text());
                let iv = validate_icon_url(&ie.text());
                let all_ok = nv && uv && iv && co;
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

        // ── Icon URL changed ────────────────────────────────
        icon_entry.connect_changed({
            let icon_ind = icon_indicator.clone();
            let ne = ne2.clone();
            let ue = ue2.clone();
            let ie = ie2.clone();
            let sb = sb2.clone();
            let sl = sl2.clone();
            let co = chrome_ok;
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
                let uv = validate_url(&ue.text());
                let iv = validate_icon_url(&ie.text());
                let all_ok = nv && uv && iv && co;
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

        // ── Submit handler ───────────────────────────────────
        submit_btn.clone().connect_clicked(move |_| {
            let app_name = name_entry.text().trim().to_string();
            let app_url = url_entry.text().trim().to_string();
            let icon_url = icon_entry.text().trim().to_string();

            submit_btn.set_sensitive(false);

            let name_c = app_name.clone();
            let url_c = app_url.clone();
            let icon_c = icon_url.clone();
            let status_c = status_label.clone();
            let submit_c = submit_btn.clone();

            gtk::glib::MainContext::default().spawn_local(async move {
                // Resolve icon path: download if URL provided, else empty
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
                match create_desktop_entry(&name_c, &url_c, &icon_path) {
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

        // ── Escape to close ─────────────────────────────────
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
