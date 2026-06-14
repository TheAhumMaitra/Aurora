// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//    Copyright (C) 2026 Ahum Maitra

//      This program is free software: you can redistribute it and/or modify
//      it under the terms of the GNU General Public License as published by
//      the Free Software Foundation, either version 3 of the License, or
//      (at your option) any later version.

//      This program is distributed in the hope that it will be useful,
//      but WITHOUT ANY WARRANTY; without even the implied warranty of
//      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//      GNU General Public License for more details.

//      You should have received a copy of the GNU General Public License
//      along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    fs::{self, OpenOptions},
    path::Path,
    process::Command,
};

const LOCK_FILE: &str = "/tmp/aurora-screensaver.lock";

fn aurora_running() -> bool {
    Command::new("pgrep")
        .args(["-f", "org.aurora.screensaver"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    // If a lock exists but Aurora is not running,
    // the lock is stale. Remove it.
    if Path::new(LOCK_FILE).exists() && !aurora_running() {
        let _ = fs::remove_file(LOCK_FILE);
    }

    // Prevent duplicate launches
    let lock = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(LOCK_FILE);

    if lock.is_err() {
        return;
    }

    // If Aurora somehow started between the stale-lock check
    // and lock acquisition, don't launch another copy.
    if aurora_running() {
        let _ = fs::remove_file(LOCK_FILE);
        return;
    }

    let result = Command::new("kitty")
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

    // Always remove the lock when Kitty exits
    let _ = fs::remove_file(LOCK_FILE);

    if let Err(err) = result {
        eprintln!("Failed to launch Aurora screensaver: {err}");
    }
}
