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

use std::{process::Command, thread, time::Duration};
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

fn main() {
    let mut sys = System::new_all();

    // Collect PIDs before refreshing so we have a clean snapshot
    sys.refresh_all();

    let waybar_pids: Vec<Pid> = sys
        .processes()
        .values()
        .filter(|p| p.name() == "waybar")
        .map(|p| p.pid())
        .collect();

    if waybar_pids.is_empty() {
        println!("No running waybar processes found.");
    } else {
        // 1. Send SIGTERM to all waybar processes
        for &pid in &waybar_pids {
            if let Some(process) = sys.process(pid) {
                println!("Stopping Waybar (PID: {pid})");
                process.kill_with(Signal::Term);
            }
        }

        // 2. Wait for all waybar processes to actually exit (with timeout)
        let max_retries = 15; // ~1.5 seconds total timeout
        for i in 0..max_retries {
            // Refresh process list to check if waybar is gone
            sys.refresh_processes(ProcessesToUpdate::All, true);

            let still_alive: Vec<Pid> = sys
                .processes()
                .values()
                .filter(|p| p.name() == "waybar")
                .map(|p| p.pid())
                .filter(|pid| waybar_pids.contains(pid))
                .collect();

            if still_alive.is_empty() {
                println!("All waybar processes stopped.");
                break;
            }

            if i == max_retries - 1 {
                // Last resort: SIGKILL
                for &pid in &still_alive {
                    if let Some(process) = sys.process(pid) {
                        println!("Waybar (PID: {pid}) not responding, sending SIGKILL");
                        process.kill_with(Signal::Kill);
                    }
                }
                thread::sleep(Duration::from_millis(100));
            } else {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    // 3. Start Waybar fresh
    println!("Starting Waybar...");
    Command::new("waybar")
        .spawn()
        .expect("Failed to start waybar");
}
