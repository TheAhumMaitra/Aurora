// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

// A deliberately small focus timer for a Hyprland desktop session.

use aurora::load_css;
use gtk4::gdk::Key;
use gtk4::glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, Entry, EventControllerKey, Label,
    Orientation, ProgressBar,
};
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;

const APP_ID: &str = "com.aurora.focus_timer";

#[derive(Clone)]
struct TimerUi {
    clock: Label,
    state_label: Label,
    progress: ProgressBar,
    start: Button,
    task: Entry,
}

#[derive(Clone, Copy)]
struct TimerState {
    duration: u32,
    remaining: u32,
    running: bool,
}

impl TimerState {
    fn new(minutes: u32) -> Self {
        let duration = minutes * 60;
        Self {
            duration,
            remaining: duration,
            running: false,
        }
    }
}

fn format_time(seconds: u32) -> String {
    format!("{:02}:{:02}", seconds / 60, seconds % 60)
}

fn update_ui(ui: &TimerUi, state: TimerState) {
    ui.clock.set_label(&format_time(state.remaining));
    ui.progress.set_fraction(
        (1.0 - state.remaining as f64 / state.duration.max(1) as f64).clamp(0.0, 1.0),
    );
    ui.start
        .set_label(if state.running { "Pause" } else { "Start" });

    let minutes = state.duration / 60;
    let description = if state.remaining == 0 {
        "Complete"
    } else if state.running {
        "In progress"
    } else {
        "Ready to focus"
    };
    ui.state_label
        .set_label(&format!("{description} · {minutes}-minute session"));
}

fn finish_session(ui: &TimerUi) {
    let task = ui.task.text().trim().to_string();
    let body = if task.is_empty() {
        "Your focus session is complete."
    } else {
        &format!("Finished: {task}")
    };
    let _ = Command::new("notify-send")
        .args(["Aurora Focus", body])
        .spawn();
}

fn set_duration(state: &Rc<RefCell<TimerState>>, ui: &TimerUi, minutes: u32) {
    let next = TimerState::new(minutes);
    *state.borrow_mut() = next;
    update_ui(ui, next);
}

fn toggle_timer(state: &Rc<RefCell<TimerState>>, ui: &TimerUi) {
    let next = {
        let mut timer = state.borrow_mut();
        if timer.remaining == 0 {
            timer.remaining = timer.duration;
        }
        timer.running = !timer.running;
        *timer
    };
    update_ui(ui, next);
}

fn preset_button(
    label: &str,
    minutes: u32,
    state: &Rc<RefCell<TimerState>>,
    ui: &Rc<TimerUi>,
) -> Button {
    let button = Button::with_label(label);
    button.add_css_class("focus-preset");
    button.connect_clicked({
        let state = state.clone();
        let ui = ui.clone();
        move |_| set_duration(&state, &ui, minutes)
    });
    button
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aurora Focus")
        .default_width(430)
        .default_height(390)
        .decorated(false)
        .resizable(false)
        .build();
    window.add_css_class("focus-timer-window");

    let title = Label::builder().label("Focus").halign(Align::Start).build();
    title.add_css_class("focus-title");
    let subtitle = Label::builder()
        .label("Give one thing your full attention")
        .halign(Align::Start)
        .build();
    subtitle.add_css_class("focus-subtitle");

    let task = Entry::builder()
        .placeholder_text("What are you focusing on?")
        .hexpand(true)
        .build();
    task.add_css_class("focus-task");

    let clock = Label::builder().halign(Align::Center).build();
    clock.add_css_class("focus-clock");
    let state_label = Label::builder().halign(Align::Center).build();
    state_label.add_css_class("focus-state");
    let progress = ProgressBar::new();
    progress.add_css_class("focus-progress");
    progress.set_show_text(false);

    let start = Button::with_label("Start");
    start.add_css_class("focus-start");
    start.set_hexpand(true);
    let reset = Button::with_label("Reset");
    reset.add_css_class("focus-reset");
    reset.set_hexpand(true);

    let controls = GtkBox::new(Orientation::Horizontal, 10);
    controls.append(&start);
    controls.append(&reset);

    let state = Rc::new(RefCell::new(TimerState::new(25)));
    let ui = Rc::new(TimerUi {
        clock,
        state_label,
        progress,
        start,
        task,
    });
    update_ui(&ui, *state.borrow());

    let presets = GtkBox::new(Orientation::Horizontal, 8);
    presets.add_css_class("focus-presets");
    presets.append(&preset_button("25 min", 25, &state, &ui));
    presets.append(&preset_button("50 min", 50, &state, &ui));
    presets.append(&preset_button("90 min", 90, &state, &ui));

    let header = GtkBox::new(Orientation::Vertical, 3);
    header.append(&title);
    header.append(&subtitle);

    let timer_card = GtkBox::new(Orientation::Vertical, 12);
    timer_card.add_css_class("focus-surface");
    timer_card.append(&ui.clock);
    timer_card.append(&ui.state_label);
    timer_card.append(&ui.progress);
    timer_card.append(&controls);

    let root = GtkBox::new(Orientation::Vertical, 14);
    root.add_css_class("focus-root");
    root.append(&header);
    root.append(&ui.task);
    root.append(&presets);
    root.append(&timer_card);
    window.set_child(Some(&root));

    ui.start.connect_clicked({
        let state = state.clone();
        let ui = ui.clone();
        move |_| toggle_timer(&state, &ui)
    });
    reset.connect_clicked({
        let state = state.clone();
        let ui = ui.clone();
        move |_| {
            let minutes = state.borrow().duration / 60;
            set_duration(&state, &ui, minutes);
        }
    });

    gtk4::glib::timeout_add_seconds_local(1, {
        let state = state.clone();
        let ui = ui.clone();
        move || {
            let (next, complete) = {
                let mut timer = state.borrow_mut();
                if !timer.running || timer.remaining == 0 {
                    (*timer, false)
                } else {
                    timer.remaining -= 1;
                    let complete = timer.remaining == 0;
                    if complete {
                        timer.running = false;
                    }
                    (*timer, complete)
                }
            };
            update_ui(&ui, next);
            if complete {
                finish_session(&ui);
            }
            ControlFlow::Continue
        }
    });

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
    ui.task.grab_focus();
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });
    app.run();
}
