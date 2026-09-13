// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use aurora::{load_css, send_notification, Reminder};
use gtk4::gdk::Key;
use gtk4::glib::{ControlFlow, SourceId};
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Entry, EventControllerKey, Label,
    Orientation,
};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "com.aurora.reminder";

#[derive(Clone)]
struct ReminderUi {
    minutes: Entry,
    task: Entry,
    schedule: Button,
    status: Label,
}

fn cancel_timers(timers: &Rc<RefCell<Vec<SourceId>>>) {
    for source in timers.borrow_mut().drain(..) {
        source.remove();
    }
}

fn set_status(status: &Label, message: &str, success: bool) {
    status.set_label(message);
    status.remove_css_class("reminder-warning");
    status.remove_css_class("reminder-success");
    status.add_css_class(if success {
        "reminder-success"
    } else {
        "reminder-warning"
    });
}

fn read_reminder(ui: &ReminderUi) -> Result<Reminder, String> {
    let minutes_text = ui.minutes.text();
    let raw_minutes = minutes_text.trim();
    if raw_minutes.is_empty() {
        return Err("Choose how many minutes from now.".to_string());
    }

    let minutes = raw_minutes
        .parse::<u64>()
        .map_err(|_| "Minutes must be a whole number, like 25.".to_string())?;
    Reminder::new(minutes, ui.task.text().trim())
}

fn schedule_reminder(ui: &ReminderUi, timers: &Rc<RefCell<Vec<SourceId>>>) {
    let reminder = match read_reminder(ui) {
        Ok(reminder) => reminder,
        Err(error) => {
            set_status(&ui.status, &error, false);
            return;
        }
    };
    let minutes = reminder.minutes;

    let task = reminder.task.clone();
    cancel_timers(timers);
    let schedule = reminder.notification_schedule();
    for (delay, title) in schedule.iter().copied() {
        let task = task.clone();
        let timers = timers.clone();
        let source = gtk4::glib::timeout_add_seconds_local(delay as u32, move || {
            send_notification(title, &task)
                .unwrap_or_else(|error| eprintln!("Could not send reminder notification: {error}"));
            ControlFlow::Break
        });
        timers.borrow_mut().push(source);
    }

    let early = match schedule.len() {
        2 => "one advance notification and final notification",
        3 => "two advance notifications and final notification",
        _ => "final notification",
    };
    set_status(
        &ui.status,
        &format!("Scheduled for {minutes} minutes · {early}"),
        true,
    );
    ui.schedule.set_label("Reschedule");
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aurora Reminder")
        .default_width(500)
        .default_height(430)
        .decorated(false)
        .resizable(false)
        .build();
    window.add_css_class("home-window");

    let title = Label::builder()
        .label("Reminder")
        .halign(Align::Center)
        .build();
    title.add_css_class("home-header");
    let subtitle = Label::builder()
        .label("Schedule a task and let Aurora notify you at the right time")
        .halign(Align::Center)
        .wrap(true)
        .build();
    subtitle.add_css_class("home-subtitle");

    let minutes = Entry::builder()
        .placeholder_text("Remind me in minutes (e.g. 25)")
        .input_purpose(gtk4::InputPurpose::Digits)
        .build();
    minutes.add_css_class("app-entry");
    let task = Entry::builder()
        .placeholder_text("What should I remind you about?")
        .build();
    task.add_css_class("app-entry");

    let schedule = Button::with_label("Schedule reminder");
    schedule.add_css_class("home-create-btn");
    let status = Label::builder()
        .label("Choose a duration to see the notification schedule.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    status.add_css_class("reminder-status");

    let ui = ReminderUi {
        minutes,
        task,
        schedule,
        status,
    };
    let timers = Rc::new(RefCell::new(Vec::new()));

    for entry in [&ui.minutes, &ui.task] {
        let ui = ui.clone();
        entry.connect_changed(move |_| {
            if ui.minutes.text().trim().is_empty() && ui.task.text().trim().is_empty() {
                ui.status
                    .set_label("You will get a warning 5 minutes before the reminder.");
                ui.status.remove_css_class("reminder-warning");
                ui.status.remove_css_class("reminder-success");
            } else {
                match read_reminder(&ui) {
                    Ok(reminder) => {
                        let count = reminder.notification_schedule().len();
                        let message = if count == 3 {
                            "Ready · two advance alerts plus the final reminder."
                        } else if count == 2 {
                            "Ready · one advance alert plus the final reminder."
                        } else {
                            "Ready · final reminder notification."
                        };
                        set_status(&ui.status, message, true);
                    }
                    Err(error) => set_status(&ui.status, &error, false),
                }
            }
        });
    }

    ui.schedule.connect_clicked({
        let ui = ui.clone();
        let timers = timers.clone();
        move |_| schedule_reminder(&ui, &timers)
    });

    let root = GtkBox::new(Orientation::Vertical, 14);
    root.add_css_class("reminder-root");
    root.set_margin_start(24);
    root.set_margin_end(24);
    root.set_margin_top(24);
    root.set_margin_bottom(24);

    let header = GtkBox::new(Orientation::Vertical, 5);
    header.add_css_class("reminder-header");
    header.append(&title);
    header.append(&subtitle);
    root.append(&header);

    let form = GtkBox::new(Orientation::Vertical, 5);
    form.add_css_class("reminder-form");
    let time_label = Label::builder()
        .label("WHEN SHOULD AURORA REMIND YOU?")
        .halign(Align::Start)
        .build();
    time_label.add_css_class("field-label");
    form.append(&time_label);
    form.append(&ui.minutes);
    let task_label = Label::builder()
        .label("WHAT SHOULD WE REMIND YOU ABOUT?")
        .halign(Align::Start)
        .build();
    task_label.add_css_class("field-label");
    form.append(&task_label);
    form.append(&ui.task);
    root.append(&form);

    let footer = GtkBox::new(Orientation::Vertical, 8);
    footer.add_css_class("reminder-footer");
    footer.append(&ui.schedule);
    footer.append(&ui.status);
    root.append(&footer);
    window.set_child(Some(&root));

    let controller = EventControllerKey::new();
    controller.connect_key_pressed({
        let window = window.clone();
        move |_, key, _, _| {
            if key == Key::Escape {
                window.close();
                return true.into();
            }
            false.into()
        }
    });
    window.add_controller(controller);
    window.present();
    ui.minutes.grab_focus();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });
    app.run();
}
