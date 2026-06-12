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
