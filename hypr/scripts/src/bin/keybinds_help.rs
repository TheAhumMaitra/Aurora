use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Grid, Label, CssProvider, Align, EventControllerKey};
use gtk::gdk::Display;
use gtk::gdk;
use gtk::ScrolledWindow;

const KEYBINDS: &[(&str, &str)] = &[
    ("Open default terminal", "SUPER + Q"),
    ("Open current window", "SUPER + C"),
    ("Exit Hyprland", "SUPER + M"),
    ("Move focus to up window", "SUPER + UP"),
    ("Move focus to down window", "SUPER + DOWN"),
    ("Move focus to right window", "SUPER + RIGHT"),
    ("Move focus to left window", "SUPER + LEFT"),
    ("Open default file manager", "SUPER + E"),
    ("Toggle window to floating", "SUPER + V"),
    ("Open scratchpad on current workspace", "SUPER + S"),
    ("Move current window to scratchpad", "SUPER + SHIFT + S"),
    ("Open default browser", "SUPER + B"),
    ("Open Hyprsettings", "SUPER + SHIFT + H"),
    ("Refresh waybar", "SUPER + W"),
    ("Toggle waybar", "SUPER + SHIFT + W"),
    ("Open web search", "SUPER + ALT + S"),
    ("Open keybinds help", "SUPER + H"),
    ("Switch to dwindle layout", "SUPER + B"),
    ("Switch to scrolling layout", "SUPER + X"),
    ("Switch to monocle layout", "SUPER + Z"),
    ("Switch to master layout", "SUPER + B")
];

fn main() {
    let app = Application::builder()
        .application_id("com.ahum.keybinds_help")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(480)
            .title("Keybinds Help")
            .decorated(false)
            .resizable(false)
            .build();

        let grid = Grid::new();
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_column_spacing(16);
        grid.set_row_spacing(8);
        grid.set_margin_start(20);
        grid.set_margin_end(20);
        grid.set_margin_top(20);
        grid.set_margin_bottom(20);
        grid.set_column_homogeneous(true);
        grid.add_css_class("keybinds-grid");

        // Headers
        let header_action = Label::builder()
            .label("Action")
            .halign(Align::Start)
            .build();
        header_action.add_css_class("header"); // <-- add CSS after building

        let header_combo = Label::builder()
            .label("Key Combo")
            .halign(Align::Start)
            .build();
        header_combo.add_css_class("header"); // <-- same here

        grid.attach(&header_action, 0, 0, 1, 1);
        grid.attach(&header_combo, 1, 0, 1, 1);

        // Keybinds
        for (row, (action, combo)) in KEYBINDS.iter().enumerate() {
            let action_label = Label::builder()
                .label(*action) // dereference &str
                .halign(Align::Start)
                .build();
            action_label.add_css_class("actions");

            let combo_label = Label::builder()
                .label(*combo) // dereference &str
                .halign(Align::Start)
                .build();
            combo_label.add_css_class("keybinds");

            grid.attach(&action_label, 0, (row + 1) as i32, 1, 1);
            grid.attach(&combo_label, 1, (row + 1) as i32, 1, 1);
        }

        let scroll = ScrolledWindow::builder()
    .hscrollbar_policy(gtk::PolicyType::Never) // optional
    .vscrollbar_policy(gtk::PolicyType::Automatic)
    .build();

        scroll.set_child(Some(&grid));
        window.set_child(Some(&scroll));
        let controller = EventControllerKey::new();

        let win = window.clone();

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

fn load_css() {
    let provider = CssProvider::new();

    // include as &str, not bytes
    provider.load_from_data(include_str!("../style.css"));

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        eprintln!("Warning: Could not connect to a display.");
    }
}