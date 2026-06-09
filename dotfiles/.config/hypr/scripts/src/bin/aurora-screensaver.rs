use std::{
    io::{self, Read},
    process::{Child, Command},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
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
        if handle.read(&mut buf).is_ok() {
            match buf[0] {
                27 | b'q' | b'Q' => {
                    // ESC = 27
                    exit_flag.store(true, Ordering::SeqCst);
                    break;
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(50));
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

    for effect in EFFECTS.iter().cycle() {
        let mut child = spawn_tarts(effect);

        let mut elapsed = 0;

        loop {
            if exit_flag.load(Ordering::SeqCst) {
                let _ = child.kill();
                let _ = child.wait();

                // restore cursor
                print!("\x1b[?25h");
                return;
            }

            thread::sleep(Duration::from_millis(100));
            elapsed += 100;

            if elapsed >= 30_000 {
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
