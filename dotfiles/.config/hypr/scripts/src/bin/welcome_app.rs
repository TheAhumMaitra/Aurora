use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, CssProvider, Align};
use gtk::gdk::Display;

fn main() {
    let app = Application::builder()
        .application_id("com.ahum.welcome")
        .build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(600)
            .default_height(400)
            .title("Welcome to Aurora")
            .decorated(false)
            .build();

        // Vertical box filling the window
        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 20);
        vbox.set_hexpand(true);
        vbox.set_vexpand(true);
        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);

        // Labels without markup
        let welcome_label = Label::builder()
            .label("Welcome to <span weight='bold' foreground='purple'>Aurora</span>")
            .halign(Align::Center)
            .use_markup(true)
            .valign(Align::Center)
            .build();
        welcome_label.add_css_class("welcome-label"); // CSS class

        let keybind_label = Label::builder()
            .label("Press <span foreground='grey'  weight='bold'>SUPER + H</span> to see all keybinds")
            .halign(Align::Center)
            .valign(Align::Center)
            .use_markup(true)
            .build();
        keybind_label.add_css_class("keybind-label"); // CSS class

        vbox.append(&welcome_label);
        vbox.append(&keybind_label);

        window.set_child(Some(&vbox));
        window.present();
    });

    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("../style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}