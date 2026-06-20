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

//main CLI for Aurora

use aurora::apply_theme;
use aurora::aurora_paths;
use aurora::download_theme;
use aurora::ghostty_blur_change;
use aurora::list_themes;
use aurora::screensaver_change;
use aurora::welcome_app_change;

use aurora::aurora_parse;

use clap::{Parser, Subcommand, ValueEnum};

use std::process::{Command, Stdio};
use which::which;

use std::fs;
const LOGO: &str = r#"
   _____                                    
  /  _  \  __ _________  ________________   
 /  /_\  \|  |  \_  __ \/  _ \_  __ \__  \  
/    |    \  |  /|  | \(  <_> )  | \// __ \_
\____|__  /____/ |__|   \____/|__|  (____  /
        \/                               \/ 
                                by Ahum Maitra :)
    "#;
/// Aurora's CLI
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Shows current Aurora's version and Aurora's cli version
    Version,
    /// Applies given theme globally
    ApplyTheme { name: String },
    /// Lists all available themes
    ListThemes,
    /// Shows information about Aurora
    Information,
    /// Returns theme details
    ThemeInfo,
    ///Downloads a theme
    DownloadTheme { git_repo_url: String },
    ///Update all external themes
    UpdateThemes,
    ///Refresh the system
    Refresh,
    ///Run specified script
    Runscript { binary_name: String },
    /// Reload Aurora with updated configuration
    Reload,
    /// Turn Aurora screensaver on or off
    Screensaver { state: ToggleState },
    /// Change Ghostty settings
    Ghostty {
        #[command(subcommand)]
        command: GhosttyCommands,
    },
    /// Change Aurora settings
    Settings {
        #[command(subcommand)]
        command: SettingsCommands,
    },
}

#[derive(Subcommand)]
enum GhosttyCommands {
    /// Turn Ghostty blur on or off
    Blur { state: ToggleState },
}

#[derive(Subcommand)]
enum SettingsCommands {
    /// Start the welcome app on Hyprland startup
    WelcomeApp { state: BooleanState },
}

#[derive(Clone, Copy, ValueEnum)]
enum ToggleState {
    On,
    Off,
}

impl ToggleState {
    fn enabled(self) -> bool {
        matches!(self, Self::On)
    }
}

#[derive(Clone, Copy, ValueEnum)]
enum BooleanState {
    True,
    False,
}

impl BooleanState {
    fn enabled(self) -> bool {
        matches!(self, Self::True)
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Version => {
            println!("{LOGO}");
            println!("Using Aurora's 0.1.0");
            println!("Using Aurora's CLI - 0.1.0");
        }
        Commands::ApplyTheme { name } => {
            apply_theme(name);
        }
        Commands::ListThemes => {
            list_themes();
        }
        Commands::Information => {
            println!(
                "{LOGO} \n Fast, minimal, beautiful Hyprland rice. This project is licensed under the terms of GPL-3.0-or-later .\n Official Repository URL :- https://github.com/TheAhumMaitra/Aurora"
            )
        }
        Commands::ThemeInfo => {
            let home = std::env::var("HOME").expect("Could not get HOME");
            let logs_directory = format!("{}/.local/share/Aurora", home);

            // write selected theme name into the theme logs file
            // Ensure the directory exists
            fs::create_dir_all(&logs_directory).expect("Failed to create logs directory");

            let theme_log_path = format!("{}/theme.log", logs_directory);

            let contents =
                fs::read_to_string(theme_log_path).expect("Failed to read the theme logs.");

            println!("Information about current theme : \n {contents}")
        }
        Commands::DownloadTheme { git_repo_url } => {
            download_theme(git_repo_url.to_string());
        }
        Commands::Refresh => {
            Command::new("refresh_system")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("failed to execute system refresh program");
        }
        Commands::Reload => match aurora_parse() {
            Ok(_) => println!("Aurora refreshed successfully."),
            Err(e) => eprintln!("Aurora error: {e}"),
        },
        Commands::Screensaver { state } => match screensaver_change(state.enabled()) {
            Ok(_) => println!(
                "Aurora screensaver {}.",
                state.to_possible_value().unwrap().get_name()
            ),
            Err(err) => {
                eprintln!("Failed to change Aurora screensaver: {err}");
                std::process::exit(1);
            }
        },
        Commands::Ghostty { command } => match command {
            GhosttyCommands::Blur { state } => match ghostty_blur_change(state.enabled()) {
                Ok(_) => println!(
                    "Ghostty blur {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change Ghostty blur: {err}");
                    std::process::exit(1);
                }
            },
        },
        Commands::Settings { command } => match command {
            SettingsCommands::WelcomeApp { state } => match welcome_app_change(state.enabled()) {
                Ok(_) => println!(
                    "Welcome app autostart set to {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change welcome app autostart: {err}");
                    std::process::exit(1);
                }
            },
        },
        Commands::UpdateThemes => {
            let paths = aurora_paths();
            let themes = paths.themes.clone();

            for entry in fs::read_dir(themes).unwrap().flatten() {
                let path = entry.path();

                if path.join(".git").exists() {
                    println!("Processing the request {}", path.display());

                    let _ = Command::new("git")
                        .arg("-C")
                        .arg(&path)
                        .arg("pull")
                        .status();
                }
            }
        }
        Commands::Runscript { binary_name } => {
            if which(binary_name).is_ok() {
                println!("Processing your request to run {binary_name}");
                Command::new(binary_name)
                    .spawn()
                    .expect("Failed to run the binary");
            } else {
                println!(
                    "Requested executable binary not found in PATH. Try to install the scripts again in PATH!"
                );
            }
        }
    }
}
