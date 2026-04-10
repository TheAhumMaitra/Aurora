use gtk::gdk;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, CssProvider, EventControllerKey, Label};
use gtk4 as gtk;
use whoami;

fn main() {
    let username = whoami::username().unwrap().to_string().to_uppercase();
    let app = Application::builder()
        .application_id("com.ahum.welcome")
        .build();

    app.connect_activate(move |app| {
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

        let wish = format!(
            "Hello <span weight='bold' foreground='violet'>{}</span>",
            username
        );
        let hello: Label = Label::builder()
            .label(&wish)
            .halign(Align::Center)
            .use_markup(true)
            .valign(Align::Center)
            .build();

        hello.add_css_class("hello");

        let welcome_label = Label::builder()
            .label("Welcome to <span weight='bold' foreground='purple'>Aurora</span>")
            .halign(Align::Center)
            .use_markup(true)
            .valign(Align::Center)
            .build();
        welcome_label.add_css_class("welcome-label"); // CSS class

        let keybind_label = Label::builder()
            .label(
                "Press <span foreground='grey'  weight='bold'>SUPER + H</span> to see all keybinds",
            )
            .halign(Align::Center)
            .valign(Align::Center)
            .use_markup(true)
            .build();
        keybind_label.add_css_class("keybind-label"); // CSS class

        vbox.append(&welcome_label);
        vbox.append(&keybind_label);
        vbox.append(&hello);

        let win = window.clone();

        window.set_child(Some(&vbox));

        let controller = EventControllerKey::new();

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
    provider.load_from_data(include_str!("../style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
