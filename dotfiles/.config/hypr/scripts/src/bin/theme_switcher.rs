use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CssProvider, EventControllerKey, Label, ListBox, ListBoxRow,
    ScrolledWindow,
};

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_paths() -> (PathBuf, PathBuf) {
    let home = std::env::var("HOME").expect("Could not get HOME");
    let themes_dir = PathBuf::from(format!("{}/.config/themes", home));
    let config_base = PathBuf::from(format!("{}/.config", home));
    (themes_dir, config_base)
}

fn list_themes() {
    let (themes_dir, _) = get_paths();

    println!("🎨 Available Themes:\n");

    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_string_lossy();
                println!("• {}", name);
            }
        }
    }
}

fn apply_theme(theme_name: &str) {
    let (themes_dir, config_base) = get_paths();

    let folders = ["waybar", "wlogout", "hypr"];
    let filenames = ["colors.css", "colors.conf"];

    println!("🎨 Applying theme: {}", theme_name);

    for folder in folders {
        let mut found = false;

        for file in filenames {
            let mut source = themes_dir.clone();
            source.push(theme_name);
            source.push(folder);
            source.push(file);

            let mut target = config_base.clone();
            target.push(folder);
            target.push(file);

            if source.exists() {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).ok();
                }

                match fs::copy(&source, &target) {
                    Ok(_) => println!("✅ Applied {}/{}", folder, file),
                    Err(e) => eprintln!("❌ Copy error in {}: {}", folder, e),
                }

                found = true;
            }
        }

        if !found {
            println!("⚠️ No colors file found in {}", folder);
        }
    }

    // 🔄 Run refresh script
    let exe = PathBuf::from(std::env::var("HOME").unwrap())
        .join(".config/hypr/scripts/target/release/refresh_system");

    Command::new(exe)
        .spawn()
        .expect("failed to run refresh_system");

    let mut wallpaper = themes_dir.clone();
    wallpaper.push(theme_name);
    wallpaper.push("default.png");

    if wallpaper.exists() {
        println!("🖼️ Setting wallpaper: {:?}", wallpaper);

        Command::new("awww")
            .args([
                "img",
                wallpaper.to_str().unwrap(),
                "--transition-type",
                "grow",
                "--transition-duration",
                "1",
            ])
            .spawn()
            .ok();
    } else {
        println!("⚠️ No default.jpg found for this theme");
    }
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
    load_css();

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::Single);

    let scroll = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .child(&list_box)
        .build();

    let (themes_dir, _) = get_paths();

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

    list_box.connect_row_activated(move |_, row| {
        let label = row.child().unwrap().downcast::<Label>().unwrap();
        let theme_name = label.text();

        apply_theme(&theme_name);
        load_css();
    });

    let controller = EventControllerKey::new();
    let win = window.clone();

    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            win.close();
            return true.into();
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

// main
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "list" => {
                if args.len() > 2 && args[2] == "themes" {
                    list_themes();
                } else {
                    println!("Usage: aurora list themes");
                }
            }

            "apply" => {
                if args.len() > 2 {
                    apply_theme(&args[2]);
                } else {
                    println!("Usage: aurora apply \"Theme Name\"");
                }
            }

            _ => {
                println!("❌ Unknown command");
            }
        }
        return;
    }

    let app = Application::builder()
        .application_id("com.aurora.theme_switcher")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
