use gtk::ScrolledWindow;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Box, Button, CssProvider};
use gtk4 as gtk;
use gtk4::Label;

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.settings")
        .build();
    
    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(480)
            .decorated(false)
            .resizable(true)
            .build();

        let title = Label::builder()
            .label("Aurora Settings")
            .valign(Align::Start)
            .halign(Align::Center)
            .build();

        title.add_css_class("settings-title");

        let scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never) // optional
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .build();

        let row = Box::new(gtk::Orientation::Horizontal, 10);

        let label = Label::new(Some("WiFi"));
        label.set_halign(gtk::Align::Start);
        label.set_hexpand(true); // 👈 THIS pushes button to right

        let button = Button::with_label("On");

        row.append(&label);
        row.append(&button);

        scroll.set_child(Some(&title));
        scroll.set_child(Some(&row));

        window.set_child(Some(&scroll));

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
