use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Label,
    ListBox, ListBoxRow, ScrolledWindow, CssProvider, EventControllerKey
};
use gtk4::gdk::Display;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.theme_switcher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Theme Switcher")
        .default_width(320)
        .default_height(420)
        .decorated(false)
        .build();
    
    window.set_opacity(0.8);

    // Load GTK UI CSS
    load_css();

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::Single);

    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&list_box)
        .build();

    // Paths
    let home = std::env::var("HOME").expect("Could not get HOME");
    let themes_dir = PathBuf::from(format!("{}/.config/themes", home));
    let config_base = PathBuf::from(format!("{}/.config", home));

    let folders = ["waybar", "wlogout"];

    // Load theme folders
    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let theme_name = path.file_name().unwrap().to_string_lossy().to_string();

                let row = ListBoxRow::new();
                let label = Label::new(Some(&theme_name));

                row.set_child(Some(&label));
                list_box.append(&row);
            }
        }
    }

    // ONLY triggers on click / Enter (NO auto trigger)
    list_box.connect_row_activated(move |_, row| {
        let label = row
            .child()
            .unwrap()
            .downcast::<Label>()
            .unwrap();

        let theme_name = label.text();

        println!("🎨 Applying theme: {}", theme_name);

        for folder in folders {
            let mut source = themes_dir.clone();
            source.push(theme_name.as_str());
            source.push(folder);
            source.push("colors.css");

            let mut target = config_base.clone();
            target.push(folder);
            target.push("colors.css");

            println!("🔍 Looking for: {:?}", source);

            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).ok();
            }

            if source.exists() {
                match fs::copy(&source, &target) {
                    Ok(_) => println!("✅ Applied {}/colors.css", folder),
                    Err(e) => eprintln!("❌ Copy error in {}: {}", folder, e),
                }
            } else {
                println!("⚠️ Missing file in {}", folder);
            }
        }

// run refresh script ONCE after all copies
let exe = std::path::PathBuf::from(std::env::var("HOME").unwrap())
    .join(".config/hypr/scripts/target/release/refresh_system");

Command::new(exe)
    .spawn()
    .expect("failed to run file");

        // reload GTK CSS after applying theme
        load_css();
    });
    let controller = EventControllerKey::new();

        let win = window.clone();

        controller.connect_key_pressed(move |_, key, _, _| {
            if key == gtk4::gdk::Key::Escape {
                // Close the window
                win.close();
                return true.into(); // event handled
            }
            false.into()
        });
        window.add_controller(controller);
    window.set_child(Some(&scroll));
    window.show();
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_data(include_str!("../style.css"));

    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}