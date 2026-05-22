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

use clap::{Parser, Subcommand, ValueEnum};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

const PID_FILE: &str = "/tmp/waybar-screenrecorder";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,

    Toggle {
        #[arg(value_enum)]
        mode: Mode,
    },

    Stop,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Fullscreen,
    Region,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => status(),
        Commands::Toggle { mode } => toggle(mode),
        Commands::Stop => stop(),
    }
}

fn status() {
    if let Ok(content) = fs::read_to_string(PID_FILE) {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() >= 3 {
            let started: u64 = lines[2].parse().unwrap_or(0);

            let elapsed = unix().saturating_sub(started);

            let mins = elapsed / 60;
            let secs = elapsed % 60;

            println!(
                r#"{{"text":"<span color='#ff0000'></span> {:02}:{:02} ","tooltip":"{}"}}"#,
                mins, secs, lines[1]
            );

            return;
        }
    }

    println!(r#"{{"text":" ","tooltip":"Stopped"}}"#);
}

fn toggle(mode: Mode) {
    if Path::new(PID_FILE).exists() {
        stop();
    } else {
        start(mode);
    }
}

fn start(mode: Mode) {
    let home = std::env::var("HOME").unwrap();

    let dir = format!("{home}/Videos/Screencasts");

    let _ = fs::create_dir_all(&dir);

    let output = Command::new("date").arg("+%Y%m%dT%H%M%S").output().unwrap();

    let timestamp = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let video = format!("{dir}/{timestamp}.mp4");

    let mut cmd = Command::new("wf-recorder");

    cmd.args(["--codec", "libx264", "--file", &video]);

    if matches!(mode, Mode::Region) {
        let area = Command::new("slurp").output().unwrap();

        let geometry = String::from_utf8_lossy(&area.stdout).trim().to_string();

        cmd.args(["--geometry", &geometry]);
    }

    let child = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    fs::write(PID_FILE, format!("{}\n{}\n{}", child.id(), video, unix())).unwrap();
}

fn stop() {
    if let Ok(content) = fs::read_to_string(PID_FILE) {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() >= 2 {
            let pid = lines[0];
            let video = lines[1];

            let _ = Command::new("kill").arg(pid).status();

            notify("Recording Saved", video);
        }
    }

    let _ = fs::remove_file(PID_FILE);
}

fn notify(title: &str, body: &str) {
    let _ = Command::new("notify-send").args([title, body]).status();
}

fn unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
