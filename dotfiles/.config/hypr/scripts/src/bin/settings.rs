use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box, Button, CssProvider, Label, Orientation,
    ScrolledWindow,
};
use std::process::Command;

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.settings")
        .build();

    app.connect_startup(|app| {
        build_ui(app);
        load_css()
    });

    app.run();
}

fn build_ui(app: &Application) {
    // Main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Settings")
        .default_width(800)
        .default_height(400)
        .build();

    window.set_opacity(0.8);

    let main_title = Label::builder()
        .label("Aurora Settings")
        .halign(Align::Start)
        .valign(Align::Center)
        .build();

    // Vertical container for all settings
    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    let home = std::env::var("HOME").unwrap();

    vbox.add_css_class("settings");

    add_setting(&vbox, "Edit main Hyprland Configuration", {
        let home = home.clone();
        move || {
            println!("Opening hyprland.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/hyprland.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit default monitor configuration", {
        let home = home.clone();
        move || {
            println!("Opening monitor.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/monitor.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit default plugins configuration", {
        let home = home.clone();
        move || {
            println!("Opening plugins.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/plugins.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit default autostart apps configuration", {
        let home = home.clone();
        move || {
            println!("Opening autostart.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/autostart.conf", home))
                .spawn()
                .unwrap();
        }
    });
    add_setting(&vbox, "Edit default keybinds", {
        let home = home.clone();
        move || {
            println!("Opening keybinds.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/keybinds.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Edit Aurora's look and feel configuration", {
        let home = home.clone();
        move || {
            println!("Opening look_and_feel.conf");

            Command::new("kitty")
                .arg("nvim")
                .arg(format!("{}/.config/hypr/configs/look_and_feel.conf", home))
                .spawn()
                .unwrap();
        }
    });

    add_setting(&vbox, "Change global theme", {
        let home = home.clone();
        move || {
            println!("Opening Theme Switcher");

            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "{}/.config/hypr/scripts/target/release/theme_switcher",
                    home
                ))
                .spawn()
                .unwrap();
        }
    });
    // Scrollable window
    let scroll = ScrolledWindow::builder()
        .min_content_height(400)
        .child(&vbox)
        .build();
    window.set_child(Some(&main_title));
    window.set_child(Some(&scroll));
    window.show();
}

fn add_setting<F: Fn() + 'static>(parent: &Box, text: &str, action: F) {
    let row = Box::new(Orientation::Horizontal, 10);
    row.add_css_class("setting-row"); // 👈 row class

    let label = Label::new(Some(text));
    label.set_xalign(0.0);
    label.set_hexpand(true);
    label.add_css_class("setting-label"); // 👈 label class

    let button = Button::with_label("Run");
    button.add_css_class("setting-button"); // 👈 button class

    button.connect_clicked(move |_| {
        action();
    });

    row.append(&label);
    row.append(&button);

    parent.append(&row);
}

fn load_css() {
    let provider = CssProvider::new();

    // include as &str, not bytes
    provider.load_from_data(include_str!("../style.css"));

    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        eprintln!("Warning: Could not connect to a display.");
    }
}
