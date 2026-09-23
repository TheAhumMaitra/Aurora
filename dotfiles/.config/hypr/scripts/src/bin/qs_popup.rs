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

use std::env;
use std::process::{exit, Command, Output, Stdio};
use std::thread;
use std::time::Duration;

/// How long a freshly started QuickShell instance may take to accept IPC calls.
const STARTUP_TIMEOUT: Duration = Duration::from_millis(5000);
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Expand a leading `~` so the tool also works when the caller did not go
/// through a shell that expands it.
fn expand_home(path: &str) -> String {
    let Some(rest) = path.strip_prefix("~/") else {
        return path.to_string();
    };

    match env::var("HOME") {
        Ok(home) => format!("{home}/{rest}"),
        Err(_) => path.to_string(),
    }
}

fn run_qs(args: &[&str]) -> Result<Output, String> {
    Command::new("qs")
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("could not run `qs`: {error}"))
}

/// `qs <config> ipc show` only succeeds while an instance of the config runs.
fn instance_is_running(config_dir: &str) -> bool {
    run_qs(&["-p", config_dir, "ipc", "show"]).is_ok_and(|output| output.status.success())
}

/// Remove ANSI escape sequences so QuickShell's diagnostics stay readable in
/// notifications and logs.
fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();

    while let Some(character) = chars.next() {
        if character != '\u{1b}' {
            out.push(character);
            continue;
        }

        // Skip a CSI sequence: `ESC [ ... <final byte in @-~>`.
        if chars.next() == Some('[') {
            for character in chars.by_ref() {
                if ('\u{40}'..='\u{7e}').contains(&character) {
                    break;
                }
            }
        }
    }

    out
}

/// A launch is only considered successful when `qs -d` neither failed nor
/// reported configuration errors: it logs errors and still exits successfully.
fn launch_error(output: &Output) -> Option<String> {
    let log = strip_ansi(&format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ));
    let errors: Vec<String> = log
        .lines()
        .filter(|line| line.contains("ERROR"))
        .map(|line| {
            line.trim()
                .trim_start_matches("ERROR:")
                .trim()
                .to_string()
        })
        .collect();

    if !errors.is_empty() {
        return Some(errors.join("\n"));
    }

    if !output.status.success() {
        let text = log.trim();
        return Some(if text.is_empty() {
            format!("`qs` exited with {}", output.status)
        } else {
            text.to_string()
        });
    }

    None
}

fn ipc_call(config_dir: &str, target: &str, op: &str) -> Result<(), String> {
    let output = run_qs(&["-p", config_dir, "ipc", "call", target, op])?;

    if !output.status.success() {
        return Err(format!("`qs ipc call` exited with {}", output.status));
    }

    // QuickShell reports unknown targets and functions on stdout without
    // failing the process, while a successful void call prints nothing.
    let message = strip_ansi(&String::from_utf8_lossy(&output.stdout))
        .trim()
        .to_string();

    if message.is_empty() {
        Ok(())
    } else {
        Err(message)
    }
}

fn fail(config_dir: &str, reason: &str) {
    eprintln!("qs_popup: {config_dir}: {reason}");

    let _ = Command::new("notify-send")
        .arg("-a")
        .arg("qs_popup")
        .arg("-u")
        .arg("critical")
        .arg("QuickShell popup failed")
        .arg(format!("{config_dir}\n{reason}"))
        .stdin(Stdio::null())
        .spawn();
}

fn call_or_fail(config_dir: &str, target: &str, op: &str) {
    if let Err(error) = ipc_call(config_dir, target, op) {
        fail(config_dir, &format!("`{target} {op}` failed: {error}"));
        exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: qs_popup <config-dir> <ipc-target> <open-op> <toggle-op>");
        exit(2);
    }

    let config_dir = expand_home(&args[1]);
    let config_dir = config_dir.trim_end_matches('/');
    let target = &args[2];
    let open_op = &args[3];
    let toggle_op = &args[4];

    if instance_is_running(config_dir) {
        call_or_fail(config_dir, target, toggle_op);
        return;
    }

    // `-d` detaches from the terminal and `-n` refuses to start a second
    // instance. The config is selected by path because that is what the caller
    // passes, and every IPC call has to use the same selection to reach it.
    let launch = match Command::new("qs")
        .args(["-d", "-n", "-p", config_dir])
        .stdin(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            fail(config_dir, &format!("could not run `qs`: {error}"));
            exit(1);
        }
    };

    if let Some(error) = launch_error(&launch) {
        fail(config_dir, &error);
        exit(1);
    }

    for _ in 0..(STARTUP_TIMEOUT.as_millis() / POLL_INTERVAL.as_millis()) {
        if instance_is_running(config_dir) {
            call_or_fail(config_dir, target, open_op);
            return;
        }
        thread::sleep(POLL_INTERVAL);
    }

    fail(
        config_dir,
        "the configuration did not come up in time, run `qs -p <config-dir>` to see why",
    );
    exit(1);
}