// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use aurora::load_css;
use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Entry, EventControllerKey, Label,
    ListBox, ListBoxRow, Orientation, ScrolledWindow, Stack,
};
use std::process::Command;

struct Action {
    icon: &'static str,
    label: &'static str,
    command: &'static str,
}

const CATEGORIES: &[(&str, &str)] = &[
    ("Apps", ""),
    ("Power", ""),
    ("Capture", ""),
    ("Utilities", ""),
    ("Aurora", ""),
    ("Help", ""),
];

const POWER: &[Action] = &[
    Action {
        icon: "",
        label: "Lock",
        command: "hyprlock",
    },
    Action {
        icon: "",
        label: "Log out",
        command: "hyprshutdown -vt 3",
    },
    Action {
        icon: "",
        label: "Suspend",
        command: "systemctl suspend",
    },
    Action {
        icon: "",
        label: "Restart",
        command: "systemctl reboot",
    },
    Action {
        icon: "",
        label: "Shutdown",
        command: "systemctl poweroff",
    },
];

const UTILITIES: &[Action] = &[
    Action {
        icon: "",
        label: "Settings",
        command: "settings",
    },
    Action {
        icon: "",
        label: "Theme switcher",
        command: "theme_switcher",
    },
    Action {
        icon: "",
        label: "Keybinds",
        command: "keybinds_help",
    },
    Action {
        icon: "",
        label: "Clock",
        command: "clock",
    },
    Action {
        icon: "",
        label: "Reminder",
        command: "reminder",
    },
    Action {
        icon: "",
        label: "Inbox",
        command: "inbox",
    },
    Action {
        icon: "",
        label: "Focus timer",
        command: "focus_timer",
    },
    Action {
        icon: "",
        label: "Workspace overview",
        command: "workspace_overview",
    },
    Action {
        icon: "",
        label: "Layout switcher",
        command: "layout_switcher",
    },
    Action {
        icon: "",
        label: "Search",
        command: "search",
    },
];

const CAPTURE: &[Action] = &[
    Action {
        icon: "",
        label: "Screenshot",
        command: "hyprshot -m output",
    },
    Action {
        icon: "",
        label: "Screenshot region",
        command: "hyprshot -m region",
    },
    Action {
        icon: "",
        label: "Screen recorder",
        command: "screenrecorder",
    },
];

const AURORA: &[Action] = &[
    Action {
        icon: "",
        label: "Keybinds help",
        command: "keybinds_help",
    },
    Action {
        icon: "",
        label: "Theme switcher",
        command: "theme_switcher",
    },
    Action {
        icon: "",
        label: "Settings",
        command: "settings",
    },
    Action {
        icon: "",
        label: "Reload Aurora",
        command: "aurora reload",
    },
];

const HELP: &[Action] = &[
    Action {
        icon: "",
        label: "Keybinds help",
        command: "keybinds_help",
    },
    Action {
        icon: "",
        label: "Aurora information",
        command: "aurora information",
    },
];

fn run(command: &str) {
    let mut parts = command.split_whitespace();
    let Some(program) = parts.next() else { return };
    if let Err(error) = Command::new(program).args(parts).spawn() {
        eprintln!("Could not launch {command}: {error}");
    }
}

fn action_panel(
    title: &'static str,
    actions: &'static [Action],
    _stack: &Stack,
    window: &ApplicationWindow,
) -> GtkBox {
    let panel = GtkBox::new(Orientation::Vertical, 6);
    let heading = Label::builder().label(title).halign(Align::Start).build();
    heading.add_css_class("unified-menu-panel-title");
    panel.append(&heading);
    for action in actions {
        let content = GtkBox::new(Orientation::Horizontal, 8);
        content.set_halign(Align::Start);
        let icon = Label::with_mnemonic(action.icon);
        icon.add_css_class("unified-menu-action-icon");
        let label = Label::with_mnemonic(action.label);
        label.set_halign(Align::Start);
        content.append(&icon);
        content.append(&label);

        let button = Button::new();
        button.set_child(Some(&content));
        button.add_css_class("unified-menu-action");
        let window = window.clone();
        button.connect_clicked(move |_| {
            run(action.command);
            window.close();
        });
        panel.append(&button);
    }
    panel
}

fn focus_category(index: usize, list: &ListBox, scroll: &ScrolledWindow) {
    let Some(row) = list.row_at_index(index as i32) else {
        return;
    };
    list.select_row(Some(&row));
    row.set_focusable(true);
    row.grab_focus();

    let adjustment = scroll.vadjustment();
    let allocation = row.allocation();
    let row_start = f64::from(allocation.y());
    let row_end = row_start + f64::from(allocation.height());
    let visible_start = adjustment.value();
    let visible_end = visible_start + adjustment.page_size();
    let target = if row_start < visible_start {
        row_start
    } else if row_end > visible_end {
        row_end - adjustment.page_size()
    } else {
        return;
    };
    let max_value = adjustment.upper() - adjustment.page_size();
    adjustment.set_value(target.clamp(adjustment.lower(), max_value.max(adjustment.lower())));
}

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.system_menu")
        .build();
    app.connect_activate(|app| {
        load_css();
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Aurora")
            .default_width(360)
            .default_height(500)
            .decorated(false)
            .resizable(false)
            .build();
        window.add_css_class("unified-menu-window");

        let stack = Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);

        let main = GtkBox::new(Orientation::Vertical, 10);
        let search = Entry::builder()
            .placeholder_text("Search anything...")
            .hexpand(true)
            .build();
        search.add_css_class("unified-menu-search");
        let list = ListBox::new();
        list.set_selection_mode(gtk4::SelectionMode::Single);
        list.add_css_class("unified-menu-list");
        list.set_vexpand(true);
        let list_scroll = ScrolledWindow::builder()
            .child(&list)
            .vexpand(true)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .build();
        list_scroll.add_css_class("unified-menu-scroll");
        for (label, icon) in CATEGORIES.iter() {
            let row = ListBoxRow::new();
            row.add_css_class("unified-menu-category-row");
            row.set_focusable(true);
            row.set_selectable(true);
            row.set_activatable(true);
            let button = Button::with_label(&format!("{icon}    {label}"));
            button.add_css_class("unified-menu-category-item");
            button.set_focusable(false);
            let page = label.to_lowercase();
            let stack = stack.clone();
            let window = window.clone();
            button.connect_clicked(move |_| {
                if page == "apps" {
                    if let Err(error) = Command::new("rofi").args(["-show", "drun"]).spawn() {
                        eprintln!("Could not launch rofi: {error}");
                    }
                    window.close();
                } else {
                    stack.set_visible_child_name(&page);
                }
            });
            row.set_child(Some(&button));
            list.append(&row);
        }
        list.connect_row_activated(|_, row| {
            if let Some(button) = row.child().and_downcast::<Button>() {
                button.activate();
            }
        });
        list.set_focusable(true);
        main.append(&search);
        main.append(&list_scroll);
        stack.add_named(&main, Some("main"));

        stack.add_named(
            &action_panel("Power", POWER, &stack, &window),
            Some("power"),
        );
        stack.add_named(
            &action_panel("Utilities", UTILITIES, &stack, &window),
            Some("utilities"),
        );
        stack.add_named(
            &action_panel("Capture", CAPTURE, &stack, &window),
            Some("capture"),
        );
        stack.add_named(
            &action_panel("Aurora", AURORA, &stack, &window),
            Some("aurora"),
        );
        stack.add_named(&action_panel("Help", HELP, &stack, &window), Some("help"));

        let root = GtkBox::new(Orientation::Vertical, 0);
        root.add_css_class("system-menu-content");
        root.set_margin_start(10);
        root.set_margin_end(10);
        root.set_margin_top(18);
        root.set_margin_bottom(18);
        root.append(&stack);
        window.set_child(Some(&root));
        let controller = EventControllerKey::new();
        controller.connect_key_pressed({
            let window = window.clone();
            let stack = stack.clone();
            let list = list.clone();
            let list_scroll = list_scroll.clone();
            let search = search.clone();
            move |_, key, _, _| {
                if key == Key::Escape {
                    if stack.visible_child_name().as_deref() == Some("main") {
                        window.close();
                    } else {
                        stack.set_visible_child_name("main");
                    }
                    return true.into();
                }
                if stack.visible_child_name().as_deref() == Some("main")
                    && search.has_focus()
                    && (key == Key::Down || key == Key::KP_Down)
                {
                    focus_category(0, &list, &list_scroll);
                    return true.into();
                }
                false.into()
            }
        });
        window.add_controller(controller);
        window.present();
        search.grab_focus();
    });
    app.run();
}
