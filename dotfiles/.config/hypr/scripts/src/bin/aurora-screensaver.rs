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

use std::process::Command;

fn set_cursor_invisible(invisible: bool) -> Result<(), String> {
    let value = if invisible { "true" } else { "false" };
    // Runs: hyprctl eval 'hl.config({ cursor = { invisible = true/false } })'
    let expression = format!("hl.config({{ cursor = {{ invisible = {value} }} }})");
    let status = Command::new("hyprctl")
        .args(["eval", &expression])
        .status()
        .map_err(|err| format!("failed to run hyprctl: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("hyprctl exited with status {status}"))
    }
}

struct CursorVisibilityGuard;

impl Drop for CursorVisibilityGuard {
    fn drop(&mut self) {
        if let Err(err) = set_cursor_invisible(false) {
            eprintln!("Failed to restore cursor visibility: {err}");
        }
    }
}

fn main() {
    if let Err(err) = set_cursor_invisible(true) {
        eprintln!("Failed to hide cursor: {err}");
    }
    // Restores `invisible = false` when termflix exits (including early
    // returns / panics). Explicit `drop` before return is not needed since
    // this is the end of `main`, but the guard guarantees the restore.
    let _cursor_visibility_guard = CursorVisibilityGuard;

    let result = Command::new("termflix")
        .args([
            "--clean",
            "--cycle",
            "15",
            "--screensaver",
            "--screensaver-keys",
        ])
        .status();

    // Explicitly drop the guard *before* returning so the
    // `hyprctl eval 'hl.config({ cursor = { invisible = false } })'`
    // restore happens deterministically before process exit.
    drop(_cursor_visibility_guard);

    if let Err(err) = result {
        eprintln!("Failed to launch termflix screensaver: {err}");
    }
}
