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

// main CLI for Aurora

use aurora::apply_theme;
use aurora::aurora_parse;
use aurora::aurora_paths;
use aurora::aurora_switcher_change;
use aurora::aurora_switcher_is_enabled;
use aurora::dock_change;
use aurora::dock_is_enabled;
use aurora::download_theme;
use aurora::ghostty_theme_blur_change;
use aurora::ghostty_theme_blur_is_enabled;
use aurora::ghostty_theme_change;
use aurora::ghostty_theme_is_enabled;
use aurora::horror_survey_game;
use aurora::kitty_blur_change;
use aurora::kitty_theme_blur_is_enabled;
use aurora::kitty_theme_change;
use aurora::kitty_theme_is_enabled;
use aurora::line_changer;
use aurora::list_themes;
use aurora::matugen_switcher_change;
use aurora::matugen_switcher_is_enabled;
use aurora::screensaver_change;
use aurora::screensaver_is_enabled;
use aurora::theme_entries;
use aurora::welcome_app_change;
use aurora::welcome_app_is_enabled;

use clap::{Parser, Subcommand, ValueEnum};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use which::which;

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

    /// Downloads a theme
    DownloadTheme { git_repo_url: String },

    /// Update all external themes
    UpdateThemes,

    /// Refresh the system
    Refresh,

    /// Run specified script
    Runscript { binary_name: String },

    /// Open the Aurora reminder app
    Reminder,

    /// Open the Aurora unified command center
    Menu {
        #[command(subcommand)]
        command: Option<MenuCommands>,
    },

    /// Reload Aurora with updated configuration
    Reload,

    /// Change Ghostty settings
    Ghostty {
        #[command(subcommand)]
        command: GhosttyCommands,
    },

    /// Change Kitty settings
    Kitty {
        #[command(subcommand)]
        command: KittyCommands,
    },

    /// Change Aurora settings
    Settings {
        #[command(subcommand)]
        command: SettingsCommands,
    },

    /// Play one of Aurora's little games
    Game {
        #[command(subcommand)]
        command: GamesCommands,
    },

    /// Change a line of a file into another line
    #[command(name = "line_changer")]
    LineChanger {
        /// File to change
        #[arg(long, value_name = "PATH")]
        file: PathBuf,

        /// Line to look for, spaces and indentation are ignored
        #[arg(long, value_name = "TEXT", conflicts_with_all = ["line_no", "line_numbers"])]
        line: Option<String>,

        /// Line number to change, counted from 1
        #[arg(long, value_name = "NUMBER", conflicts_with_all = ["line", "line_numbers"])]
        line_no: Option<String>,

        /// Lines to change, counted from 1, such as 137-183
        #[arg(long, value_name = "FIRST-LAST", conflicts_with_all = ["line", "line_no"])]
        line_numbers: Option<String>,

        /// Text the line becomes, an empty text deletes the line
        #[arg(long, value_name = "TEXT")]
        line_to: Option<String>,
    },

    /// Check the setup has ALL FEATURES AND EVERYTHING OF AURORA
    #[command(name = "doctar", alias = "doctor")]
    Doctar {
        /// Show verbose output with full details for all checks
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum GhosttyCommands {
    /// Turn Ghostty theme management on or off
    Theme { state: ToggleState },

    /// Turn Ghostty blur theme on or off (only applies when theme is on)
    Blur { state: ToggleState },
}

#[derive(Subcommand)]
enum KittyCommands {
    /// Turn Kitty theme management on or off
    Theme { state: ToggleState },

    /// Turn Kitty blur theme on or off (only applies when theme is on)
    Blur { state: ToggleState },
}

#[derive(Subcommand)]
enum SettingsCommands {
    /// Start the welcome app on Hyprland startup
    WelcomeApp { state: BooleanState },

    /// Start nwg-dock-hyprland on Hyprland startup
    Dock { state: BooleanState },

    /// Turn Aurora screensaver on or off
    Screensaver { state: ToggleState },

    /// Pick which wallpaper switcher keybind stays enabled
    #[command(name = "theme-switcher")]
    ThemeSwitcher {
        #[command(subcommand)]
        command: ThemeSwitcherCommands,
    },
}

#[derive(Subcommand)]
enum ThemeSwitcherCommands {
    /// Keep Aurora's own theme wallpaper switcher, commenting out matugen's one
    Aurora { state: BooleanState },

    /// Keep the matugen based wallpaper switcher, commenting out Aurora's one
    Matugen { state: BooleanState },
}

#[derive(Subcommand)]
enum GamesCommands {
    /// Horror and dread themed games
    Horror {
        #[command(subcommand)]
        command: HorrorCommands,
    },
}

#[derive(Subcommand)]
enum MenuCommands {
    /// Open the command center
    Open,
    /// Toggle the command center
    Toggle,
}

#[derive(Subcommand)]
enum HorrorCommands {
    /// The "Are your windows closed?" horror survey
    Survey,
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

/// Both switchers can not be bound at once, so picking one always comments the
/// other out.
fn counterpart_note(enabled: bool) -> &'static str {
    if enabled {
        "commented out"
    } else {
        "enabled instead"
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
                "{LOGO}\nFast, minimal, beautiful Hyprland rice. This project is licensed under the terms of GPL-3.0-or-later.\nOfficial Repository URL :- https://github.com/TheAhumMaitra/Aurora"
            );
        }

        Commands::ThemeInfo => {
            let home = std::env::var("HOME").expect("Could not get HOME");
            let logs_directory = format!("{}/.local/share/Aurora", home);

            fs::create_dir_all(&logs_directory).expect("Failed to create logs directory");

            let theme_log_path = format!("{}/theme.log", logs_directory);

            let contents =
                fs::read_to_string(theme_log_path).expect("Failed to read the theme logs.");

            println!("Information about current theme:\n{contents}");
        }

        Commands::DownloadTheme { git_repo_url } => {
            download_theme(git_repo_url.to_string());
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

        Commands::Refresh => {
            Command::new("refresh_system")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("failed to execute system refresh program");
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

        Commands::Reminder => {
            Command::new("reminder")
                .spawn()
                .expect("Failed to run the reminder app");
        }

        Commands::Menu { .. } => {
            Command::new("system_menu")
                .spawn()
                .expect("Failed to run the Aurora command center");
        }

        Commands::Reload => match aurora_parse() {
            Ok(_) => println!("Aurora refreshed successfully."),
            Err(e) => eprintln!("Aurora error: {e}"),
        },

        Commands::Ghostty { command } => match command {
            GhosttyCommands::Theme { state } => match ghostty_theme_change(state.enabled()) {
                Ok(_) => println!(
                    "Ghostty theme {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change Ghostty theme: {err}");
                    std::process::exit(1);
                }
            },
            GhosttyCommands::Blur { state } => match ghostty_theme_blur_change(state.enabled()) {
                Ok(_) => println!(
                    "Ghostty blur theme {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change Ghostty blur: {err}");
                    std::process::exit(1);
                }
            },
        },

        Commands::Kitty { command } => match command {
            KittyCommands::Theme { state } => match kitty_theme_change(state.enabled()) {
                Ok(_) => println!(
                    "Kitty theme management {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change Kitty theme: {err}");
                    std::process::exit(1);
                }
            },
            KittyCommands::Blur { state } => match kitty_blur_change(state.enabled()) {
                Ok(_) => println!(
                    "Kitty blur theme {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change Kitty blur theme: {err}");
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

            SettingsCommands::Screensaver { state } => match screensaver_change(state.enabled()) {
                Ok(_) => println!(
                    "Aurora screensaver {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change Aurora screensaver: {err}");
                    std::process::exit(1);
                }
            },

            SettingsCommands::Dock { state } => match dock_change(state.enabled()) {
                Ok(_) => println!(
                    "Dock autostart set to {}.",
                    state.to_possible_value().unwrap().get_name()
                ),
                Err(err) => {
                    eprintln!("Failed to change dock autostart: {err}");
                    std::process::exit(1);
                }
            },

            SettingsCommands::ThemeSwitcher { command } => match command {
                ThemeSwitcherCommands::Aurora { state } => {
                    match aurora_switcher_change(state.enabled()) {
                        Ok(_) => println!(
                            "Aurora wallpaper switcher set to {}, the matugen one is {}.",
                            state.to_possible_value().unwrap().get_name(),
                            counterpart_note(state.enabled())
                        ),
                        Err(err) => {
                            eprintln!("Failed to change the Aurora wallpaper switcher: {err}");
                            std::process::exit(1);
                        }
                    }
                }

                ThemeSwitcherCommands::Matugen { state } => {
                    match matugen_switcher_change(state.enabled()) {
                        Ok(_) => println!(
                            "Matugen wallpaper switcher set to {}, Aurora's one is {}.",
                            state.to_possible_value().unwrap().get_name(),
                            counterpart_note(state.enabled())
                        ),
                        Err(err) => {
                            eprintln!("Failed to change the matugen wallpaper switcher: {err}");
                            std::process::exit(1);
                        }
                    }
                }
            },
        },

        Commands::Game { command } => match command {
            GamesCommands::Horror { command } => match command {
                HorrorCommands::Survey => match horror_survey_game() {
                    Ok(_) => println!("You step back from the window, still breathing."),
                    Err(err) => {
                        eprintln!("The survey went wrong: {err}");
                        std::process::exit(1);
                    }
                },
            },
        },

        Commands::LineChanger {
            file,
            line,
            line_no,
            line_numbers,
            line_to,
        } => {
            let numbers = line_no.as_deref().or(line_numbers.as_deref());

            match line_changer(file, line.as_deref(), numbers, line_to.as_deref()) {
                Ok(changed) => {
                    let line = if changed == 1 { "line" } else { "lines" };

                    println!("Changed {changed} {line} of {}.", file.display());
                }
                Err(err) => {
                    eprintln!("Failed to change the line: {err}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Doctar { verbose } => {
            run_doctor(*verbose);
        }
    }
}

const C_RESET: &str = "\x1b[0m";
const C_BOLD: &str = "\x1b[1m";
const C_DIM: &str = "\x1b[90m";
const C_RED: &str = "\x1b[31m";
const C_GREEN: &str = "\x1b[32m";
const C_YELLOW: &str = "\x1b[33m";
const C_CYAN: &str = "\x1b[36m";

#[derive(Clone, Copy, PartialEq, Eq)]
enum CheckLevel {
    Required,
    Recommended,
    Optional,
}

#[derive(Default)]
struct DoctorReport {
    passed: usize,
    warnings: usize,
    failures: usize,
}

fn print_section(title: &str) {
    println!("\n{C_BOLD}{title}{C_RESET}");
}

fn check_binary(
    name: &str,
    description: &str,
    level: CheckLevel,
    report: &mut DoctorReport,
    verbose: bool,
) -> bool {
    match which(name) {
        Ok(path) => {
            report.passed += 1;
            if verbose {
                println!(
                    "  {C_GREEN}✓{C_RESET} {name} {C_DIM}— {description} ({}){C_RESET}",
                    path.display()
                );
            } else {
                println!("  {C_GREEN}✓{C_RESET} {name} {C_DIM}— {description}{C_RESET}");
            }
            true
        }
        Err(_) => {
            match level {
                CheckLevel::Required => {
                    report.failures += 1;
                    println!(
                        "  {C_RED}✗{C_RESET} {name} {C_DIM}— {description}{C_RESET} {C_RED}[missing]{C_RESET}"
                    );
                }
                CheckLevel::Recommended => {
                    report.warnings += 1;
                    println!(
                        "  {C_YELLOW}!{C_RESET} {name} {C_DIM}— {description}{C_RESET} {C_YELLOW}[recommended]{C_RESET}"
                    );
                }
                CheckLevel::Optional => {
                    if verbose {
                        println!("  {C_DIM}– {name} — {description} [optional]{C_RESET}");
                    }
                }
            }
            false
        }
    }
}

fn check_binary_alternatives(
    names: &[&str],
    display_name: &str,
    description: &str,
    level: CheckLevel,
    report: &mut DoctorReport,
    verbose: bool,
) -> bool {
    for name in names {
        if let Ok(path) = which(name) {
            report.passed += 1;
            if verbose {
                println!(
                    "  {C_GREEN}✓{C_RESET} {display_name} {C_DIM}— {description} (found: {}){C_RESET}",
                    path.display()
                );
            } else {
                println!(
                    "  {C_GREEN}✓{C_RESET} {display_name} {C_DIM}— {description} ({name}){C_RESET}"
                );
            }
            return true;
        }
    }

    match level {
        CheckLevel::Required => {
            report.failures += 1;
            println!(
                "  {C_RED}✗{C_RESET} {display_name} {C_DIM}— {description}{C_RESET} {C_RED}[missing]{C_RESET}"
            );
        }
        CheckLevel::Recommended => {
            report.warnings += 1;
            println!(
                "  {C_YELLOW}!{C_RESET} {display_name} {C_DIM}— {description}{C_RESET} {C_YELLOW}[recommended]{C_RESET}"
            );
        }
        CheckLevel::Optional => {
            if verbose {
                println!("  {C_DIM}– {display_name} — {description} [optional]{C_RESET}");
            }
        }
    }
    false
}

fn check_file(
    path: &Path,
    label: &str,
    level: CheckLevel,
    report: &mut DoctorReport,
    verbose: bool,
) {
    if path.exists() {
        report.passed += 1;
        if verbose {
            println!("  {C_GREEN}✓{C_RESET} {label} {C_DIM}({}){C_RESET}", path.display());
        } else {
            println!("  {C_GREEN}✓{C_RESET} {label}");
        }
    } else {
        match level {
            CheckLevel::Required => {
                report.failures += 1;
                println!("  {C_RED}✗{C_RESET} {label} {C_RED}[missing]{C_RESET}");
            }
            CheckLevel::Recommended => {
                report.warnings += 1;
                println!("  {C_YELLOW}!{C_RESET} {label} {C_YELLOW}[not found]{C_RESET}");
            }
            CheckLevel::Optional => {}
        }
    }
}

fn check_daemon(process_name: &str, service_label: &str, report: &mut DoctorReport) {
    let running = Command::new("pgrep")
        .arg("-x")
        .arg(process_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if running {
        report.passed += 1;
        println!("  {C_GREEN}✓{C_RESET} {service_label} {C_DIM}[running]{C_RESET}");
    } else {
        report.warnings += 1;
        println!("  {C_YELLOW}!{C_RESET} {service_label} {C_DIM}[not running]{C_RESET}");
    }
}

fn run_doctor(verbose: bool) {
    let mut report = DoctorReport::default();

    println!("{C_BOLD}Aurora Doctor{C_RESET} {C_DIM}— Checking setup and features{C_RESET}");

    // 1. Session & Environment
    print_section("Session & Environment");
    if let Ok(disp) = std::env::var("WAYLAND_DISPLAY") {
        report.passed += 1;
        println!("  {C_GREEN}✓{C_RESET} Wayland display: {disp}");
    } else {
        report.warnings += 1;
        println!("  {C_YELLOW}!{C_RESET} Wayland display: not detected in current shell");
    }

    if std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some() {
        report.passed += 1;
        println!("  {C_GREEN}✓{C_RESET} Hyprland session: active");
    } else {
        let running = Command::new("pgrep")
            .arg("-x")
            .arg("Hyprland")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if running {
            report.passed += 1;
            println!("  {C_GREEN}✓{C_RESET} Hyprland session: process running");
        } else {
            report.warnings += 1;
            println!("  {C_YELLOW}!{C_RESET} Hyprland session: not running");
        }
    }

    let paths = aurora_paths();
    let themes_dir = &paths.themes;
    let themes = theme_entries();
    if themes.is_empty() {
        report.warnings += 1;
        println!("  {C_YELLOW}!{C_RESET} Themes: none found in {}", themes_dir.display());
    } else {
        report.passed += 1;
        println!(
            "  {C_GREEN}✓{C_RESET} Themes: {} installed {C_DIM}({}){C_RESET}",
            themes.len(),
            themes_dir.display()
        );
        if verbose {
            for t in &themes {
                println!("    {C_DIM}• {}{C_RESET}", t.display_name);
            }
        }
    }

    let home = std::env::var("HOME").unwrap_or_default();
    let theme_log = PathBuf::from(&home).join(".local/share/Aurora/theme_name.log");
    if theme_log.exists() {
        if let Ok(name) = fs::read_to_string(&theme_log) {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                report.passed += 1;
                println!("  {C_GREEN}✓{C_RESET} Active theme: {trimmed}");
            }
        }
    }

    // 2. Core Hyprland Desktop Stack
    print_section("Core Hyprland Desktop Stack");
    check_binary("hyprland", "Dynamic tiling Wayland compositor", CheckLevel::Required, &mut report, verbose);
    check_binary("hypridle", "Hyprland idle management daemon", CheckLevel::Required, &mut report, verbose);
    check_binary("hyprlock", "GPU-accelerated screen locker", CheckLevel::Required, &mut report, verbose);
    check_binary("hyprshot", "Hyprland screenshot utility", CheckLevel::Required, &mut report, verbose);
    check_binary("hyprshutdown", "Graceful logout / shutdown helper", CheckLevel::Recommended, &mut report, verbose);
    check_binary("snappy-switcher", "Workspace switcher & window overviewer", CheckLevel::Recommended, &mut report, verbose);
    check_binary_alternatives(&["hyprmoncfgd", "hyprmon"], "hyprmon", "Automated monitor configuration daemon", CheckLevel::Recommended, &mut report, verbose);

    // 3. Shell, Bar & UI Services
    print_section("Aurora Shell, Bar & UI Services");
    check_binary_alternatives(&["qs", "quickshell"], "quickshell (qs)", "Quickshell desktop UI framework (aurora-shell)", CheckLevel::Required, &mut report, verbose);
    check_binary("waybar", "Status bar", CheckLevel::Required, &mut report, verbose);
    check_binary("swaync", "Sway Notification Center daemon", CheckLevel::Required, &mut report, verbose);
    check_binary("swaync-client", "SwayNC CLI client", CheckLevel::Required, &mut report, verbose);
    check_binary("swayosd-server", "SwayOSD volume/brightness daemon", CheckLevel::Recommended, &mut report, verbose);
    check_binary("swayosd-client", "SwayOSD client", CheckLevel::Recommended, &mut report, verbose);
    check_binary("rofi", "Application launcher & menu daemon", CheckLevel::Required, &mut report, verbose);
    check_binary("wlogout", "Wayland logout menu", CheckLevel::Required, &mut report, verbose);
    check_binary("waytrogen", "Wallpaper manager", CheckLevel::Required, &mut report, verbose);
    check_binary("wallust", "Dynamic wallpaper colorscheme generator", CheckLevel::Required, &mut report, verbose);
    check_binary("matugen", "Material You color palette generator", CheckLevel::Recommended, &mut report, verbose);
    check_binary_alternatives(&["awww", "awww-daemon"], "awww", "Wallpaper daemon", CheckLevel::Required, &mut report, verbose);
    check_binary("cliphist", "Clipboard history manager", CheckLevel::Required, &mut report, verbose);
    check_binary("wl-copy", "Clipboard copy tool", CheckLevel::Required, &mut report, verbose);
    check_binary("wl-paste", "Clipboard paste tool", CheckLevel::Required, &mut report, verbose);

    // 4. Terminals, Applications & Media
    print_section("Terminals, Applications & Utilities");
    check_binary("kitty", "Primary terminal emulator", CheckLevel::Recommended, &mut report, verbose);
    check_binary("ghostty", "Secondary terminal emulator", CheckLevel::Recommended, &mut report, verbose);
    check_binary("nvim", "Default text editor", CheckLevel::Recommended, &mut report, verbose);
    check_binary("nemo", "Graphical file manager", CheckLevel::Recommended, &mut report, verbose);
    check_binary_alternatives(&["zen-browser", "firefox", "chromium", "google-chrome-stable", "brave"], "browser", "Default web browser", CheckLevel::Recommended, &mut report, verbose);
    check_binary("starship", "Prompt", CheckLevel::Recommended, &mut report, verbose);
    check_binary("btop", "Resource monitor", CheckLevel::Recommended, &mut report, verbose);
    check_binary("leenfetch", "System information fetch tool", CheckLevel::Recommended, &mut report, verbose);
    check_binary("fish", "Interactive shell", CheckLevel::Recommended, &mut report, verbose);
    check_binary("wpctl", "WirePlumber audio control", CheckLevel::Required, &mut report, verbose);
    check_binary("brightnessctl", "Screen brightness controller", CheckLevel::Recommended, &mut report, verbose);
    check_binary("playerctl", "Media player controller", CheckLevel::Recommended, &mut report, verbose);
    check_binary_alternatives(&["hyprwave", "hyprwave-toggle"], "hyprwave", "Music player wave widget", CheckLevel::Optional, &mut report, verbose);

    // 5. Aurora Built-in Tools & Binaries
    print_section("Aurora Built-in Binaries");
    let builtin_tools: [(&str, &str); 34] = [
        ("app_entries_home", "Aurora App Entries center"),
        ("aurora-launch-screensaver", "Screensaver launcher"),
        ("aurora-lock", "Aurora lock wrapper"),
        ("aurora-screensaver", "Aurora screensaver"),
        ("aurora-shell", "Aurora Quickshell shell"),
        ("clock", "Aurora desktop clock"),
        ("command_palette", "Unified command palette"),
        ("focus_timer", "Pomodoro / focus timer GUI"),
        ("inbox", "Quick inbox / notes keeper"),
        ("installer", "Package & dependency installer"),
        ("keybinds_help", "Hyprland cheatsheet popup"),
        ("layout_switcher", "Hyprland layout switcher"),
        ("qs_popup", "Quickshell popup helper"),
        ("quick_note", "Fast scratchpad note taker"),
        ("refresh_system", "Instant system reloader"),
        ("reminder", "Desktop reminder scheduler"),
        ("remover", "Aurora package remover"),
        ("rofi_config_switcher", "Rofi style switcher"),
        ("screenrecorder", "Screen recorder widget"),
        ("search", "Desktop web/app search popup"),
        ("settings", "Aurora settings GUI"),
        ("starship_switcher", "Starship prompt theme switcher"),
        ("system_menu", "Unified system control menu"),
        ("theme_switcher", "Theme switcher GUI"),
        ("tui_app_entries_center", "TUI apps launcher"),
        ("tui_app_entry_creator", "TUI desktop entry generator"),
        ("waybar_flavour_switcher", "Waybar theme flavour selector"),
        ("waybar_position_switcher", "Waybar position switcher"),
        ("waybar_refresh", "Waybar reload helper"),
        ("waybar_toggle", "Waybar hide/show toggle"),
        ("web_app_entries_center", "Web apps manager"),
        ("web_app_entry_creator", "Web app desktop shortcut creator"),
        ("welcome_app", "Aurora first-launch welcome app"),
        ("workspace_overview", "Hyprland workspace visualizer"),
    ];

    let mut missing_builtins = Vec::new();
    let mut found_builtins = 0;
    for (bin, desc) in &builtin_tools {
        if which(bin).is_ok() {
            found_builtins += 1;
            report.passed += 1;
            if verbose {
                println!("  {C_GREEN}✓{C_RESET} {bin} {C_DIM}— {desc}{C_RESET}");
            }
        } else {
            missing_builtins.push((*bin, *desc));
            report.failures += 1;
        }
    }

    if !verbose {
        if missing_builtins.is_empty() {
            println!(
                "  {C_GREEN}✓{C_RESET} All {}/{} Aurora built-in binaries in PATH",
                found_builtins,
                builtin_tools.len()
            );
        } else {
            println!(
                "  {C_RED}✗{C_RESET} {}/{} Aurora built-in binaries found in PATH",
                found_builtins,
                builtin_tools.len()
            );
            for (bin, desc) in &missing_builtins {
                println!("    {C_RED}✗{C_RESET} {bin} {C_DIM}— {desc}{C_RESET} {C_RED}[missing]{C_RESET}");
            }
        }
    }

    // 6. Aurora Dotfiles & Configurations
    print_section("Aurora Dotfiles & Configurations");
    let config_dir = &paths.config;
    let config_files: [(PathBuf, &str, CheckLevel); 21] = [
        (config_dir.join("hypr/hyprland.lua"), "hyprland.lua", CheckLevel::Required),
        (config_dir.join("hypr/hypridle.conf"), "hypridle.conf", CheckLevel::Required),
        (config_dir.join("hypr/hyprlock.conf"), "hyprlock.conf", CheckLevel::Required),
        (config_dir.join("hypr/configs/autostart.lua"), "autostart.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/keybinds.lua"), "keybinds.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/animations.lua"), "animations.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/window_rules.lua"), "window_rules.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/look_and_feel.lua"), "look_and_feel.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/input.lua"), "input.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/env_vars.lua"), "env_vars.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/layer_rules.lua"), "layer_rules.lua", CheckLevel::Required),
        (config_dir.join("hypr/configs/layout_configs.lua"), "layout_configs.lua", CheckLevel::Required),
        (config_dir.join("waybar/style.css"), "waybar/style.css", CheckLevel::Required),
        (config_dir.join("swaync/config.json"), "swaync/config.json", CheckLevel::Required),
        (config_dir.join("swaync/style.css"), "swaync/style.css", CheckLevel::Required),
        (config_dir.join("rofi/config.rasi"), "rofi/config.rasi", CheckLevel::Required),
        (config_dir.join("kitty/kitty.conf"), "kitty/kitty.conf", CheckLevel::Recommended),
        (config_dir.join("ghostty/config.ghostty"), "ghostty/config.ghostty", CheckLevel::Recommended),
        (config_dir.join("wallust/wallust.toml"), "wallust/wallust.toml", CheckLevel::Required),
        (config_dir.join("aurora/palette.toml"), "aurora/palette.toml", CheckLevel::Required),
        (config_dir.join("starship.toml"), "starship.toml", CheckLevel::Recommended),
    ];

    for (path, label, level) in &config_files {
        check_file(path, label, *level, &mut report, verbose);
    }

    // 7. Running Daemons & Services
    print_section("Active Services");
    check_daemon("awww-daemon", "awww-daemon", &mut report);
    check_daemon("waybar", "waybar", &mut report);
    check_daemon("swaync", "swaync", &mut report);
    check_daemon("hypridle", "hypridle", &mut report);
    check_daemon("swayosd-server", "swayosd-server", &mut report);
    check_daemon("hyprmoncfgd", "hyprmoncfgd", &mut report);
    check_daemon("snappy-switcher", "snappy-switcher", &mut report);

    // 8. Aurora Feature Settings State
    print_section("Feature Toggles");
    let welcome_status = match welcome_app_is_enabled() {
        Ok(true) => "enabled",
        _ => "disabled",
    };
    println!("  {C_DIM}•{C_RESET} Welcome app: {welcome_status}");

    let dock_status = match dock_is_enabled() {
        Ok(true) => "enabled",
        _ => "disabled",
    };
    println!("  {C_DIM}•{C_RESET} Dock autostart: {dock_status}");

    let screensaver_status = match screensaver_is_enabled() {
        Ok(true) => "enabled",
        _ => "disabled",
    };
    println!("  {C_DIM}•{C_RESET} Screensaver: {screensaver_status}");

    let aurora_switcher = aurora_switcher_is_enabled().unwrap_or(false);
    let matugen_switcher = matugen_switcher_is_enabled().unwrap_or(false);
    let switcher_status = if aurora_switcher {
        "aurora native"
    } else if matugen_switcher {
        "matugen mixture"
    } else {
        "default"
    };
    println!("  {C_DIM}•{C_RESET} Wallpaper switcher: {switcher_status}");

    if let Ok(enabled) = kitty_theme_is_enabled() {
        let blur = kitty_theme_blur_is_enabled().unwrap_or(false);
        println!(
            "  {C_DIM}•{C_RESET} Kitty theming: {} (blur: {})",
            if enabled { "managed" } else { "unmanaged" },
            if blur { "on" } else { "off" }
        );
    }
    if let Ok(enabled) = ghostty_theme_is_enabled() {
        let blur = ghostty_theme_blur_is_enabled().unwrap_or(false);
        println!(
            "  {C_DIM}•{C_RESET} Ghostty theming: {} (blur: {})",
            if enabled { "managed" } else { "unmanaged" },
            if blur { "on" } else { "off" }
        );
    }

    // 9. Diagnostics Summary
    print_section("Diagnostics Summary");
    println!(
        "  Passed: {C_GREEN}{}{C_RESET} | Warnings: {C_YELLOW}{}{C_RESET} | Failures: {C_RED}{}{C_RESET}",
        report.passed, report.warnings, report.failures
    );

    if report.failures == 0 && report.warnings == 0 {
        println!("\n{C_GREEN}✓ Setup has all features and everything of Aurora in pristine condition.{C_RESET}\n");
    } else if report.failures == 0 {
        println!(
            "\n{C_GREEN}✓ All required features of Aurora are present and functional.{C_RESET} {C_DIM}({} non-critical item(s) could be tuned){C_RESET}\n",
            report.warnings
        );
    } else {
        println!(
            "\n{C_RED}✗ Aurora has {} missing required item(s).{C_RESET}",
            report.failures
        );
        println!("  {C_DIM}You can install missing packages with:{C_RESET}");
        println!("  aurora runscript installer --pacman <pkg> --aur <pkg>\n");
    }
}
