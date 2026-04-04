use sysinfo::{System, Signal};
use std::{process::Command, thread, time::Duration};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // 1. Kill all waybar processes
    for process in sys.processes().values() {
        if process.name() == "waybar" {
            println!("Stopping Waybar (PID: {})", process.pid());
            process.kill_with(Signal::Term); // graceful stop
        }
    }

    // 2. Small delay (important!)
    thread::sleep(Duration::from_millis(200));

    // 3. Start Waybar again
    println!("Starting Waybar...");
    Command::new("waybar")
        .spawn()
        .expect("Failed to start waybar");
}