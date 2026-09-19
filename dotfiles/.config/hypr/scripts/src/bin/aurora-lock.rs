//  SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com> */
//  SPDX-License-Identifier: GPL-3.0-or-later */
//    Copyright (C) 2026 Ahum Maitra

//       This program is free software: you can redistribute it and/or modify
//       it under the terms of the GNU General Public License as published by
//       the Free Software Foundation, either version 3 of the License, or
//       (at your option) any later version.

//       This program is distributed in the hope that it will be useful,
//       but WITHOUT ANY WARRANTY; without even the implied warranty of
//       MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//       GNU General Public License for more details.

//       You should have received a copy of the GNU General Public License
//       along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{process::Command, thread, time::Duration};

fn set_cursor_invisible(invisible: bool) {
    let value = if invisible { "true" } else { "false" };
    // Runs: hyprctl eval 'hl.config({ cursor = { invisible = true/false } })'
    let expression = format!("hl.config({{ cursor = {{ invisible = {value} }} }})");
    if let Err(err) = Command::new("hyprctl").args(["eval", &expression]).status() {
        eprintln!("Failed to restore cursor visibility: {err}");
    }
}

fn main() {
    let _ = Command::new("pkill")
        .args(["-f", "org.aurora.screensaver"])
        .status();
    let _ = Command::new("pkill")
        .args(["-f", "aurora-screensaver"])
        .status();

    thread::sleep(Duration::from_millis(300));

    // aurora-launch-screensaver restores this on exit via its guard, but a
    // SIGKILL/SIGTERM kill bypasses Drop — always restore here so the cursor
    // can't stay stuck invisible after locking.
    set_cursor_invisible(false);

    if let Err(err) = Command::new("hyprlock").spawn() {
        eprintln!("Failed to launch hyprlock: {err}");
    }
}
