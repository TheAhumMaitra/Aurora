use std::path::PathBuf;
use std::process::Command;

fn run(exe: &PathBuf) {
    println!("Running: {:?}", exe);

    match Command::new(exe).status() {
        Ok(status) => println!("Exited: {:?}", status),
        Err(e) => eprintln!("Failed: {}", e),
    }
}

fn main() {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/release");

    let scripts = vec![base.join("waybar_refresh")];

    for script in &scripts {
        run(script);
    }
    Command::new("hyprctl")
        .args(["reload"])
        .output()
        .expect("failed to execute process");
}
