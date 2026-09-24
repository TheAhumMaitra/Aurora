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
use std::process::{Command, Output, Stdio, exit};
use std::thread;
use std::time::Duration;

/// How long a freshly started QuickShell instance may take to accept IPC calls.
const STARTUP_TIMEOUT: Duration = Duration::from_millis(5000);
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// A QuickShell instance that controls Aurora's popups.
fn config_dir() -> Result<String, String> {
    let home = env::var("HOME").map_err(|_| "HOME is not set".to_string())?;

    Ok(match env::var("XDG_CONFIG_HOME") {
        Ok(base) if !base.is_empty() => format!("{base}/quickshell/aurora-shell"),
        _ => format!("{home}/.config/quickshell/aurora-shell"),
    })
}

fn run_qs(config_dir: &str, args: &[&str]) -> Result<Output, String> {
    Command::new("qs")
        .arg("-p")
        .arg(config_dir)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("could not run `qs`: {error}"))
}

/// `qs <config> ipc show` only succeeds while an instance of the config runs.
fn instance_is_running(config_dir: &str) -> bool {
    run_qs(config_dir, &["ipc", "show"]).is_ok_and(|output| output.status.success())
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
        .map(|line| line.trim().trim_start_matches("ERROR:").trim().to_string())
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

/// Make sure the Aurora QuickShell instance is up, starting it if needed.
fn ensure_running(config_dir: &str) -> Result<(), String> {
    if instance_is_running(config_dir) {
        return Ok(());
    }

    // `-d` detaches from the terminal and `-n` refuses to start a second
    // instance. Starting Aurora alone shows nothing; popups only appear when
    // an IPC call maps them.
    let launch = Command::new("qs")
        .args(["-d", "-n", "-p", config_dir])
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("could not run `qs`: {error}"))?;

    if let Some(error) = launch_error(&launch) {
        return Err(error);
    }

    for _ in 0..(STARTUP_TIMEOUT.as_millis() / POLL_INTERVAL.as_millis()) {
        if instance_is_running(config_dir) {
            return Ok(());
        }
        thread::sleep(POLL_INTERVAL);
    }

    Err("the instance did not start, run `qs -p <config-dir>` to see why".to_string())
}

/// Accept any of the accepted component names and map it to its canonical form
/// plus the IPC target exposed by the manager's `IpcHandler`.
struct Component {
    /// Canonical name used by the `aurora` IPC handler (open/close/toggle).
    name: &'static str,
    /// IPC target of the manager itself, used by the `call` subcommand.
    target: &'static str,
}

fn component(name: &str) -> Option<Component> {
    match name {
        "bluetooth" | "bluetooth_manager" | "bt" => Some(Component {
            name: "bluetooth",
            target: "bluetooth",
        }),
        "network" | "networks" | "networks_manager" | "net" | "wifi" => Some(Component {
            name: "network",
            target: "wifi",
        }),
        "volume" | "volume_adjuster" | "vol" => Some(Component {
            name: "volume",
            target: "volume",
        }),
        "control" | "control_center" | "cc" | "dashboard" | "quick" => Some(Component {
            name: "control_center",
            target: "cc",
        }),
        _ => None,
    }
}

fn require_component(name: &str) -> Result<Component, String> {
    component(name).ok_or_else(|| {
        format!(
            "unknown component '{name}'\n  try: bluetooth_manager, networks_manager, volume_adjuster or control_center"
        )
    })
}

/// Run `qs ipc call` and surface what QuickShell prints for unknown targets or
/// functions (it reports them on stdout without failing the process).
fn ipc_call(config_dir: &str, args: &[&str]) -> Result<(), String> {
    let mut full: Vec<&str> = vec!["ipc", "call"];
    full.extend_from_slice(args);

    let output = run_qs(config_dir, &full)?;

    if !output.status.success() {
        return Err(format!("`qs ipc call` exited with {}", output.status));
    }

    let message = strip_ansi(&String::from_utf8_lossy(&output.stdout))
        .trim()
        .to_string();

    if message.is_empty() {
        Ok(())
    } else {
        Err(message)
    }
}

fn open(config_dir: &str, component: &Component) -> Result<(), String> {
    ensure_running(config_dir)?;
    ipc_call(config_dir, &["aurora", "open", component.name])
}

fn close(config_dir: &str, component: &Component) -> Result<(), String> {
    ensure_running(config_dir)?;
    ipc_call(config_dir, &["aurora", "close", component.name])
}

fn toggle(config_dir: &str, component: &Component) -> Result<(), String> {
    ensure_running(config_dir)?;
    ipc_call(config_dir, &["aurora", "toggle", component.name])
}

fn cmd_start(config_dir: &str, args: &[String]) -> Result<(), String> {
    let Some(name) = args.first() else {
        // No component given: just start the Aurora instance, show no popup.
        return ensure_running(config_dir);
    };

    open(config_dir, &require_component(name)?)
}

fn cmd_stop(config_dir: &str, args: &[String]) -> Result<(), String> {
    let Some(name) = args.first() else {
        return Err(
            "unknown component ''\n  try: bluetooth_manager, networks_manager, volume_adjuster or control_center"
                .to_string(),
        );
    };

    if name == "all" {
        for canonical in ["bluetooth", "network", "volume", "control_center"] {
            close(config_dir, &require_component(canonical)?)?;
        }
        return Ok(());
    }

    close(config_dir, &require_component(name)?)
}

fn cmd_toggle(config_dir: &str, args: &[String]) -> Result<(), String> {
    let Some(name) = args.first() else {
        return Err(
            "unknown component ''\n  try: bluetooth_manager, networks_manager, volume_adjuster or control_center"
                .to_string(),
        );
    };

    toggle(config_dir, &require_component(name)?)
}

fn cmd_call(config_dir: &str, args: &[String]) -> Result<(), String> {
    let Some(name) = args.first() else {
        return Err(
            "unknown component ''\n  try: bluetooth_manager, networks_manager, volume_adjuster or control_center"
                .to_string(),
        );
    };
    let index = require_component(name)?;

    let Some(function) = args.get(1) else {
        return Err("'call' needs a function name".to_string());
    };

    let mut words: Vec<&str> = vec![index.target, function];
    words.extend(args.iter().skip(2).map(String::as_str));

    ensure_running(config_dir)?;
    ipc_call(config_dir, &words)
}

fn cmd_status(config_dir: &str) -> Result<(), String> {
    let output = run_qs(config_dir, &["ipc", "show"])?;
    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn usage() {
    println!(
        "Aurora Shell control.

  aurora-shell start  <component>   open a popup (starting Aurora if needed)
  aurora-shell stop   <component>   close a popup
  aurora-shell toggle <component>   toggle a popup
  aurora-shell <component>          same as `toggle`
  aurora-shell call <component> <function> [args...]
  aurora-shell start                just start the Aurora instance
  aurora-shell status               list the available IPC targets

Components: bluetooth_manager | networks_manager | volume_adjuster | control_center
(shorthands: bluetooth/bt, network/net/wifi, volume/vol, control/cc/dashboard/quick)

Starting Aurora alone shows nothing; popups are only mapped when opened."
    );
}

fn fail(message: &str) {
    eprintln!("aurora-shell: {message}");
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let Some(command) = args.first() else {
        usage();
        exit(2);
    };

    let config_dir = match config_dir() {
        Ok(dir) => dir,
        Err(error) => {
            eprintln!("aurora-shell: {error}");
            exit(2);
        }
    };

    let rest = &args[1..];
    let result = match command.as_str() {
        "start" => cmd_start(&config_dir, rest),
        "stop" | "close" => cmd_stop(&config_dir, rest),
        "toggle" => cmd_toggle(&config_dir, rest),
        "call" => cmd_call(&config_dir, rest),
        "status" => cmd_status(&config_dir),
        "help" | "-h" | "--help" => {
            usage();
            return;
        }
        name => cmd_toggle(&config_dir, &[name.to_string()]),
    };

    match result {
        Ok(()) => {}
        Err(error) => fail(&error),
    }
}
