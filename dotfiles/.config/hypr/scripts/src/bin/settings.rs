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
use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Label,
    Orientation, ScrolledWindow, SearchEntry,
};
use std::process::Command;

struct SettingsCategory {
    title: &'static str,
    description: &'static str,
    items: &'static [SettingsItem],
}

struct SettingsItem {
    title: &'static str,
    description: &'static str,
    button: &'static str,
    action: SettingsAction,
}

#[derive(Clone, Copy)]
enum SettingsAction {
    #[allow(dead_code)]
    Edit(&'static str),
    Launch(&'static str),
    Run(&'static str, &'static [&'static str]),
    Open(&'static str),
}

const SETTINGS_CATEGORIES: &[SettingsCategory] = &[
    SettingsCategory {
        title: "Appearance",
        description: "Change the desktop style, theme, wallpaper, and shell look.",
        items: &[
            SettingsItem {
                title: "Theme switcher",
                description: "Apply global theme",
                button: "Open",
                action: SettingsAction::Launch("theme_switcher"),
            },
            SettingsItem {
                title: "Theme based wallpaper switcher",
                description: "Open wallpaper switcher, which will show theme's wallpapers",
                button: "Open",
                action: SettingsAction::Launch("waytrogen_line_change_for_theme"),
            },
            SettingsItem {
                title: "Global wallpaper switcher",
                description: "Open all wallpaper switcher for global wallpapers",
                button: "Open",
                action: SettingsAction::Launch("waytrogen_line_change_for_global_wallpapers"),
            },
        ],
    },
    SettingsCategory {
        title: "Aurora Tools",
        description: "Open Aurora helper apps and quick utilities",
        items: &[
            SettingsItem {
                title: "Keybinds help",
                description: "See all keybinds",
                button: "Open",
                action: SettingsAction::Launch("keybinds_help"),
            },
            SettingsItem {
                title: "Search popup",
                description: "Open search bar for web search",
                button: "Open",
                action: SettingsAction::Launch("search"),
            },
            SettingsItem {
                title: "Layout switcher",
                description: "This will switch Hyprland layouts",
                button: "Open",
                action: SettingsAction::Launch("layout_switcher"),
            },
            SettingsItem {
                title: "Refresh system",
                description: "Refresh the whole system",
                button: "Run",
                action: SettingsAction::Launch("refresh_system"),
            },
        ],
    },
    SettingsCategory {
        title: "System",
        description: "Session maintenance actions for Hyprland and Waybar.",
        items: &[
            SettingsItem {
                title: "Reload Hyprland",
                description: "Reload Hyprland's configs",
                button: "Reload",
                action: SettingsAction::Run("hyprctl", &["reload"]),
            },
            SettingsItem {
                title: "Refresh Waybar",
                description: "Restart Waybar",
                button: "Refresh",
                action: SettingsAction::Launch("waybar_refresh"),
            },
            SettingsItem {
                title: "Toggle Waybar",
                description: "Turn on or off Waybar",
                button: "Toggle",
                action: SettingsAction::Launch("waybar_toggle"),
            },
        ],
    },
    SettingsCategory {
        title: "Folders",
        description: "Open config directories",
        items: &[
            SettingsItem {
                title: "Hyprland folder",
                description: "Open ~/.config/hypr in your file manager.",
                button: "Open",
                action: SettingsAction::Open(".config/hypr"),
            },
            SettingsItem {
                title: "Themes folder",
                description: "Open ~/.config/themes.",
                button: "Open",
                action: SettingsAction::Open(".config/themes"),
            },
            SettingsItem {
                title: "Waybar folder",
                description: "Open ~/.config/waybar.",
                button: "Open",
                action: SettingsAction::Open(".config/waybar"),
            },
            SettingsItem {
                title: "Aurora scripts",
                description: "Open the Rust scripts source folder.",
                button: "Open",
                action: SettingsAction::Open(".config/hypr/scripts/src/bin"),
            },
        ],
    },
];

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.settings")
        .build();

    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aurora Settings")
        .default_width(760)
        .default_height(560)
        .decorated(false)
        .resizable(true)
        .build();

    window.set_opacity(0.8);

    let list_box = GtkBox::new(Orientation::Vertical, 12);
    list_box.add_css_class("settings-list");
    append_settings(&list_box, "");

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .child(&list_box)
        .build();

    let title = Label::builder()
        .label("Aurora Settings")
        .halign(Align::Start)
        .build();
    title.add_css_class("settings-title");

    let subtitle = Label::builder()
        .label("Categorized controls for Hyprland, themes, Aurora tools, and session maintenance.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    subtitle.add_css_class("settings-subtitle");

    let search_entry = SearchEntry::builder()
        .placeholder_text("Search settings...")
        .hexpand(true)
        .build();
    search_entry.add_css_class("search-entry");

    let search_controller = EventControllerKey::new();
    let win_ref = window.clone();

    search_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            win_ref.close();
            return true.into();
        }
        false.into()
    });

    search_entry.add_controller(search_controller);

    search_entry.connect_search_changed({
        let list_box = list_box.clone();
        move |entry| append_settings(&list_box, &entry.text())
    });

    let header = GtkBox::new(Orientation::Vertical, 6);
    header.add_css_class("settings-header");
    header.append(&title);
    header.append(&subtitle);
    header.append(&search_entry);

    let root = GtkBox::new(Orientation::Vertical, 0);
    root.add_css_class("settings-root");
    root.append(&header);
    root.append(&scroll);

    let controller = EventControllerKey::new();
    let win = window.clone();

    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk::Key::Escape {
            win.close();
            return true.into();
        }
        false.into()
    });

    window.add_controller(controller);
    window.set_child(Some(&root));
    window.present();
}

fn append_settings(parent: &GtkBox, query: &str) {
    while let Some(child) = parent.first_child() {
        parent.remove(&child);
    }

    let query = query.trim().to_lowercase();
    let mut has_results = false;

    for category in SETTINGS_CATEGORIES {
        let category_matches = category.title.to_lowercase().contains(&query)
            || category.description.to_lowercase().contains(&query);

        let matching_items: Vec<_> = category
            .items
            .iter()
            .filter(|item| {
                query.is_empty()
                    || category_matches
                    || item.title.to_lowercase().contains(&query)
                    || item.description.to_lowercase().contains(&query)
                    || item.button.to_lowercase().contains(&query)
            })
            .collect();

        if matching_items.is_empty() {
            continue;
        }

        has_results = true;
        add_category(parent, category, &matching_items);
    }

    if !has_results {
        let empty = Label::builder()
            .label("No settings matched your search.")
            .halign(Align::Center)
            .valign(Align::Center)
            .vexpand(true)
            .build();
        empty.add_css_class("settings-empty");
        parent.append(&empty);
    }
}

fn add_category(parent: &GtkBox, category: &SettingsCategory, items: &[&SettingsItem]) {
    let section = GtkBox::new(Orientation::Vertical, 8);
    section.add_css_class("settings-category");

    let title = Label::builder()
        .label(category.title)
        .halign(Align::Start)
        .build();
    title.add_css_class("settings-category-title");

    let description = Label::builder()
        .label(category.description)
        .halign(Align::Start)
        .wrap(true)
        .build();
    description.add_css_class("settings-category-description");

    section.append(&title);
    section.append(&description);

    for item in items {
        add_setting(&section, item);
    }

    parent.append(&section);
}

fn add_setting(parent: &GtkBox, item: &SettingsItem) {
    let row = GtkBox::new(Orientation::Horizontal, 12);
    row.add_css_class("setting-row");

    let copy = GtkBox::new(Orientation::Vertical, 3);
    copy.set_hexpand(true);

    let label = Label::new(Some(item.title));
    label.set_xalign(0.0);
    label.set_wrap(true);
    label.add_css_class("setting-label");

    let description = Label::new(Some(item.description));
    description.set_xalign(0.0);
    description.set_wrap(true);
    description.add_css_class("setting-description");

    copy.append(&label);
    copy.append(&description);

    let button = Button::with_label(item.button);
    button.add_css_class("setting-button");

    button.connect_clicked({
        let action = item.action;
        move |_| run_action(&action)
    });

    row.append(&copy);
    row.append(&button);

    parent.append(&row);
}

fn run_action(action: &SettingsAction) {
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from(""));

    let result = match action {
        SettingsAction::Edit(path) => Command::new("kitty")
            .arg("nvim")
            .arg(format!("{home}/{path}"))
            .spawn(),
        SettingsAction::Launch(command) => Command::new(command).spawn(),
        SettingsAction::Run(command, args) => Command::new(command).args(*args).spawn(),
        SettingsAction::Open(path) => Command::new("xdg-open")
            .arg(format!("{home}/{path}"))
            .spawn(),
    };

    if let Err(err) = result {
        eprintln!("Failed to run settings action: {err}");
    }
}
