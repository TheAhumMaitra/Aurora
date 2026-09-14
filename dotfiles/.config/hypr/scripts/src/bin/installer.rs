// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::Parser;

const AURORA_ART: &str = r#"
   ▄████████ ███    █▄     ▄████████  ▄██████▄     ▄████████    ▄████████ 
  ███    ███ ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    ███ ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    ███ ███    ███  ▄███▄▄▄▄██▀ ███    ███  ▄███▄▄▄▄██▀   ███    ███ 
▀███████████ ███    ███ ▀▀███▀▀▀▀▀   ███    ███ ▀▀███▀▀▀▀▀   ▀███████████ 
  ███    ███ ███    ███ ▀███████████ ███    ███ ▀███████████   ███    ███ 
  ███    ███ ███    ███   ███    ███ ███    ███   ███    ███   ███    ███ 
  ███    █▀  ████████▀    ███    ███  ▀██████▀    ███    ███   ███    █▀  
                          ███    ███              ███    ███               
"#;

#[derive(Debug, Default)]
struct Packages {
    cargo: Vec<String>,
    pnpm: Vec<String>,
    pacman: Vec<String>,
    aur: Vec<String>,
}

#[derive(Debug, Parser)]
#[command(
    name = "installer",
    about = "Install packages using Cargo, pnpm, pacman, or an AUR helper"
)]
struct Cli {
    #[arg(long = "cargo", value_name = "PACKAGE", num_args = 1..)]
    cargo: Vec<String>,

    #[arg(long = "pnpm", value_name = "PACKAGE", num_args = 1..)]
    pnpm: Vec<String>,

    #[arg(long = "pacman", value_name = "PACKAGE", num_args = 1..)]
    pacman: Vec<String>,

    #[arg(long = "aur", value_name = "PACKAGE", num_args = 1..)]
    aur: Vec<String>,

    #[arg(long, hide = true)]
    in_terminal: bool,
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let cli = Cli::parse();

    if !cli.in_terminal {
        if let Err(error) = open_terminal(&arguments) {
            eprintln!("Could not open Kitty: {error}");
            std::process::exit(1);
        }
        return;
    }

    let packages = Packages {
        cargo: cli.cargo,
        pnpm: cli.pnpm,
        pacman: cli.pacman,
        aur: cli.aur,
    };

    if packages.cargo.is_empty()
        && packages.pnpm.is_empty()
        && packages.pacman.is_empty()
        && packages.aur.is_empty()
    {
        println!("{AURORA_ART}");
        println!("No packages selected.");
        println!();
        println!("Usage:");
        println!("  installer --cargo <package>");
        println!("  installer --pnpm <package>");
        println!("  installer --pacman <package>");
        println!("  installer --aur <package>");
        println!("\nPress Enter to close this terminal...");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        return;
    }

    println!("{AURORA_ART}");
    println!("Installing package");

    let mut failed = false;
    failed |= install("cargo", &packages.cargo, |package| {
        ("cargo", vec!["install".to_owned(), package.to_owned()])
    });
    failed |= install_with_output(
        "pnpm",
        &packages.pnpm,
        |package| {
            (
                "pnpm",
                vec!["add".to_owned(), "--global".to_owned(), package.to_owned()],
            )
        },
        true,
    );
    failed |= install_with_output(
        "pacman",
        &packages.pacman,
        |package| {
            (
                "sudo",
                vec![
                    "pacman".to_owned(),
                    "--sync".to_owned(),
                    "--noconfirm".to_owned(),
                    package.to_owned(),
                ],
            )
        },
        true,
    );
    failed |= install_aur(&packages.aur);

    if failed {
        println!("\nOne or more packages could not be installed.");
    } else {
        println!("\nPackage installation complete.");
    }
    println!("Press Enter to close this terminal...");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    if failed {
        std::process::exit(1);
    }
}

fn open_terminal(arguments: &[String]) -> io::Result<()> {
    let executable = env::current_exe()?;
    let mut kitty_arguments = vec![
        "--class".to_owned(),
        "com.aurora.installer".to_owned(),
        "-e".to_owned(),
        executable.to_string_lossy().into_owned(),
    ];
    kitty_arguments.push("--in-terminal".to_owned());
    kitty_arguments.extend(
        arguments
            .iter()
            .filter(|argument| *argument != "--in-terminal")
            .cloned(),
    );

    let status = Command::new("kitty").args(kitty_arguments).status()?;
    if !status.success() {
        eprintln!("Kitty exited with {status}");
    }
    Ok(())
}

fn install<F>(manager: &str, packages: &[String], command_for: F) -> bool
where
    F: Fn(&str) -> (&str, Vec<String>),
{
    install_with_output(manager, packages, command_for, false)
}

fn install_with_output<F>(
    manager: &str,
    packages: &[String],
    command_for: F,
    interactive: bool,
) -> bool
where
    F: Fn(&str) -> (&str, Vec<String>),
{
    let mut failed = false;
    for package in packages {
        let (program, arguments) = command_for(package);
        if !run_command(program, &arguments, interactive) {
            eprintln!("Failed to install {package} with {manager}.");
            failed = true;
        }
    }
    failed
}

fn install_aur(packages: &[String]) -> bool {
    let Some(manager) = ["yay", "paru"].iter().find(|manager| which(*manager)) else {
        if !packages.is_empty() {
            eprintln!("AUR installation requires yay or paru.");
        }
        return !packages.is_empty();
    };

    install_with_output(
        "AUR",
        packages,
        |package| {
            (
                manager,
                vec![
                    "--sync".to_owned(),
                    "--noconfirm".to_owned(),
                    package.to_owned(),
                ],
            )
        },
        true,
    )
}

fn run_command(program: &str, arguments: &[String], interactive: bool) -> bool {
    let mut command = Command::new(program);
    command.args(arguments);

    if program == "pnpm" {
        let pnpm_home = env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(".local/share/pnpm"));
        if let Some(pnpm_home) = pnpm_home {
            let mut path = pnpm_home.clone().into_os_string();
            path.push(":");
            path.push(env::var_os("PATH").unwrap_or_default());
            command.env("PNPM_HOME", pnpm_home).env("PATH", path);
        }
    }

    if !interactive {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }

    command
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn which(program: &str) -> bool {
    Command::new("sh")
        .args([
            "-c",
            "command -v \"$1\" >/dev/null 2>&1",
            "installer",
            program,
        ])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
