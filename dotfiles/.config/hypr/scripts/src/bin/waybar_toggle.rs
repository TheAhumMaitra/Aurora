use sysinfo::{System, Signal};
use std::process::Command;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut is_running = false;

    // Check if Waybar is running
    for process in sys.processes().values() {
        if process.name() == "waybar" {
            println!("Stopping Waybar (PID: {})", process.pid());
            process.kill_with(Signal::Term); // graceful stop
            is_running = true;
        }
    }

    // If not running → start it
    if !is_running {
        println!("Starting Waybar...");
        Command::new("waybar")
            .spawn()
            .expect("Failed to start waybar");
    }
}