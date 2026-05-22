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
    env,
    fs,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

const PID: &str = "/tmp/screenrecorder";

fn main() {
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("status") => status(),
        Some("stop") => stop(),
        Some("region") => start(true),
        _ => start(false),
    }
}

fn status() {
    if let Ok(data) = fs::read_to_string(PID) {
        let v: Vec<&str> = data.lines().collect();

        if v.len() >= 3 {
            let secs =
                now() - v[2].parse::<u64>().unwrap_or(0);

            println!(
                r#"{{"text":"<span color='#ff0000'> </span> {:02}:{:02}  ","tooltip":"{}"}}"#,
                secs / 60,
                secs % 60,
                v[1]
            );

            return;
        }
    }

    println!(r#"{{"text":""}}"#);
}

fn start(region: bool) {
    if fs::metadata(PID).is_ok() {
        return;
    }

    let home = env::var("HOME").unwrap();

    let dir =
        format!("{home}/Videos/Screencasts");

    let _ = fs::create_dir_all(&dir);

    let time = String::from_utf8_lossy(
        &Command::new("date")
            .arg("+%Y%m%dT%H%M%S")
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .to_string();

    let file = format!("{dir}/{time}.mp4");

    let mut cmd = Command::new("wf-recorder");

    cmd.args([
        "--codec",
        "libx264",
        "--file",
        &file,
    ]);

    if region {
        let geo = String::from_utf8_lossy(
            &Command::new("slurp")
                .output()
                .unwrap()
                .stdout,
        )
        .trim()
        .to_string();

        cmd.args(["--geometry", &geo]);
    }

    let child = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    fs::write(
        PID,
        format!(
            "{}\n{}\n{}",
            child.id(),
            file,
            now()
        ),
    )
    .unwrap();
}

fn stop() {
    if let Ok(data) = fs::read_to_string(PID) {
        let v: Vec<&str> = data.lines().collect();

        if v.len() >= 2 {
            let _ = Command::new("kill")
                .arg(v[0])
                .status();

            let _ = Command::new("notify-send")
                .args([
                    "Recording Saved",
                    v[1],
                ])
                .status();
        }
    }

    let _ = fs::remove_file(PID);
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}