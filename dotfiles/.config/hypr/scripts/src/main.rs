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
use aurora::list_themes;

use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

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
    }
}
