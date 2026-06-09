use std::{fs, process::Command};

fn main() {
    let lock = "/tmp/aurora-screensaver.lock";

    // HARD GUARD: exit immediately if already running
    if fs::metadata(lock).is_ok() {
        return;
    }

    // create lock BEFORE anything else
    let _ = fs::write(lock, "running");

    // spawn screensaver (BLOCKING)
    let status = Command::new("kitty")
        .args([
            "--config",
            "NONE",
            "--class",
            "org.aurora.screensaver",
            "--override",
            "background=#000000",
            "--override",
            "font_size=16",
            "--override",
            "window_padding_width=0",
            "-e",
            "aurora-screensaver",
        ])
        .status();

    // cleanup ALWAYS (even crash)
    let _ = fs::remove_file(lock);

    let _ = status;
}
