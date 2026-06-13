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
    io::{self, Read},
    process::{Child, Command},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

const EFFECTS: &[&str] = &[
    "matrix",
    "life",
    "maze",
    "boids",
    "cube",
    "crab",
    "donut",
    "pipes",
    "plasma",
    "fire",
    "terrain",
    "constellation",
];

fn spawn_tarts(effect: &str) -> Child {
    Command::new("tarts")
        .arg(effect)
        .spawn()
        .expect("failed to start tarts")
}

fn input_listener(exit_flag: Arc<AtomicBool>) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0u8; 1];

    loop {
        match handle.read(&mut buf) {
            Ok(1) => {
                exit_flag.store(true, Ordering::SeqCst);
                break;
            }
            Ok(0) => break,
            Err(_) => break,
            _ => {}
        }
    }
}

fn main() {
    // hide cursor
    print!("\x1b[?25l");

    let exit_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = exit_flag.clone();

    thread::spawn(move || {
        input_listener(flag_clone);
    });

    let start = Instant::now();

    for effect in EFFECTS.iter().cycle() {
        let mut child = spawn_tarts(effect);

        let mut elapsed = 0;

        loop {
            // Exit after 2 minutes
            if start.elapsed() >= Duration::from_secs(450) {
                let _ = child.kill();
                let _ = child.wait();
                print!("\x1b[?25h");
                return;
            }

            if exit_flag.load(Ordering::SeqCst) {
                let _ = child.kill();
                let _ = child.wait();
                print!("\x1b[?25h");
                return;
            }

            thread::sleep(Duration::from_millis(100));
            elapsed += 100;

            if elapsed >= 15_000 {
                break;
            }

            if let Ok(Some(_)) = child.try_wait() {
                break;
            }
        }

        let _ = child.kill();
        let _ = child.wait();
    }

    print!("\x1b[?25h");
}
