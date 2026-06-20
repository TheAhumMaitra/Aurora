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

use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::io::Read;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;

use gtk4 as gtk;
use gtk4::CssProvider;
use gtk4::gdk::Display;

pub struct AuroraPaths {
    pub home: PathBuf,
    pub config: PathBuf,
    pub themes: PathBuf,
}

impl AuroraPaths {
    pub fn new() -> Self {
        let home = dirs::home_dir().expect("Could not get HOME directory");
        let config = home.join(".config");

        Self {
            themes: config.join("themes"),
            home,
            config,
        }
    }
}

pub fn aurora_paths() -> AuroraPaths {
    AuroraPaths::new()
}

#[derive(Debug, Clone)]
pub struct ThemeEntry {
    pub directory_name: String,
    pub display_name: String,
    pub preview_path: PathBuf,
}

fn read_theme_config(theme_dir: &Path) -> Option<Config> {
    let config_path = theme_dir.join("config.toml");
    fs::read_to_string(config_path)
        .ok()
        .and_then(|content| toml::from_str(&content).ok())
}

pub fn theme_entries() -> Vec<ThemeEntry> {
    let paths = aurora_paths();
    let mut themes = Vec::new();

    if let Ok(entries) = fs::read_dir(&paths.themes) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let directory_name = path.file_name().unwrap().to_string_lossy().to_string();
            let display_name = read_theme_config(&path)
                .map(|config| config.name)
                .filter(|name| !name.trim().is_empty())
                .unwrap_or_else(|| directory_name.clone());

            themes.push(ThemeEntry {
                directory_name,
                display_name,
                preview_path: path.join("preview.png"),
            });
        }
    }

    themes.sort_by(|a, b| {
        a.display_name
            .to_lowercase()
            .cmp(&b.display_name.to_lowercase())
    });
    themes
}

// list all available themes in `~/.config/themes` for mostly cli
pub fn list_themes() {
    println!("Showing Available Themes:\n");

    for theme in theme_entries() {
        println!("• {}", theme.display_name);
    }
}

// Root structure for parsing config.toml
#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    version: String,
    authors: Vec<String>,
    repo_url: Option<String>,
    wallpapers_sources: Option<Vec<String>>,
    license: Option<String>,
    settings: Option<Settings>,
    gtk: Option<GtkOptions>,
    vscode: Option<VsCodeOptions>,
    zed: Option<ZedOptions>,
}

// get options inside of the theme configuration file's settings category (we only need required options)
#[derive(Deserialize, Debug)]
struct Settings {
    script: Option<String>,
    interpreter: Option<String>,
}

// get zed settinsg from theme configuration file
#[derive(Deserialize, Debug)]
struct ZedOptions {
    theme_name: String,
}

// get options inside of the theme configuration file's settings GTK (we only need required options)
#[derive(Deserialize, Debug)]
struct GtkOptions {
    theme_name: Option<String>,
    icon_theme: Option<String>,
}

#[derive(Deserialize, Debug)]
struct VsCodeOptions {
    publisher: String,
    extension_name: String,
    theme_name: String,
}

fn theme_debug(message: impl AsRef<str>) {
    println!("[theme-switcher] {}", message.as_ref());
}

// apply the selected theme
pub fn apply_theme(theme_name: &str) {
    theme_debug(format!("Starting theme switch for `{theme_name}`"));

    let paths = aurora_paths();
    let aurora_data = paths.home.join(".local/share/Aurora");
    let hypr_theme_file = paths.config.join("hypr/Theme/theme.lua");
    theme_debug(format!(
        "Using config directory: {}",
        paths.config.display()
    ));
    theme_debug(format!(
        "Using themes directory: {}",
        paths.themes.display()
    ));

    // resolve theme directory and load its `config.toml` (if present)
    let theme_dir = paths.themes.join(theme_name);
    theme_debug(format!("Resolved theme directory: {}", theme_dir.display()));
    theme_debug("Reading theme config.toml");
    let config: Option<Config> = read_theme_config(&theme_dir);
    if config.is_some() {
        theme_debug("Loaded theme config.toml");
    } else {
        theme_debug("No readable config.toml found; using directory name as theme name");
    }

    // prefer the theme's declared name from config when available
    let display_name = config
        .as_ref()
        .map(|c| c.name.clone())
        .unwrap_or_else(|| theme_name.to_string());
    theme_debug(format!("Theme display name: {display_name}"));

    let folders = [
        "waybar", "wlogout", "hypr", "rofi", "nvim", "btop", "zed", "ghostty",
    ]; //directories want to be copied

    let message = format!("{display_name} is applied!");

    // Empty the current theme configuration for Hyprland
    theme_debug(format!(
        "Clearing Hyprland theme file: {}",
        hypr_theme_file.display()
    ));
    fs::write(&hypr_theme_file, "").expect("Failed to empty theme configuration");

    // reset any gtk theme configurations
    theme_debug("Resetting GNOME gtk-theme setting");
    Command::new("gsettings")
        .args(["reset", "org.gnome.desktop.interface", "gtk-theme"])
        .status()
        .expect("failed to reset gtk-theme");

    // set gtk colorscheme preference as dark always
    theme_debug("Setting GNOME color-scheme to prefer-dark");
    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.interface",
            "color-scheme",
            "\'prefer-dark\'",
        ])
        .status()
        .expect("Failed to set preference of gtk colorscheme");

    if let Some(config) = &config {
        theme_debug("Applying optional GTK, VS Code, and Zed settings from config.toml");
        apply_gtk_options(&paths, config);
        apply_vscode_options(&paths, config);
        apply_zed_options(&paths, config);
    } else {
        theme_debug("Skipping optional app settings because config.toml was not loaded");
    }

    // write selected theme directory into the theme name log file
    // Ensure the directory exists
    theme_debug(format!(
        "Ensuring Aurora data directory exists: {}",
        aurora_data.display()
    ));
    fs::create_dir_all(&aurora_data).expect("Failed to create logs directory");

    // Now write the file
    let theme_name_log_file = aurora_data.join("theme_name.log");

    theme_debug(format!(
        "Writing selected theme log: {}",
        theme_name_log_file.display()
    ));
    fs::write(&theme_name_log_file, theme_name)
        .expect("Failed to write theme name into the theme logs file.");
    // write theme logs
    let theme_log_file = aurora_data.join("theme.log");

    if let Some(config) = &config {
        theme_debug(format!(
            "Writing theme metadata log: {}",
            theme_log_file.display()
        ));
        let authors = config.authors.join(", ");
        let wallpaper_sources = match &config.wallpapers_sources {
            Some(sources) => sources.join(", "),
            None => "None".to_string(),
        };

        let information_about_theme = format!(
            "Theme Name = {}\nTheme Version = {}\nAuthors = {}\nRepo url = {}\nLicense = {}\nWallpaper sources = {}",
            config.name,
            config.version,
            authors,
            config.repo_url.as_deref().unwrap_or("None"),
            config.license.as_deref().unwrap_or("None"),
            wallpaper_sources
        );

        fs::write(&theme_log_file, information_about_theme).expect("Failed to write theme info");
    } else {
        theme_debug("Skipping theme metadata log because config.toml was not loaded");
    }

    println!("Applying theme : {}", display_name);

    // Get the options from valid categories and do the job
    if let Some(config) = &config {
        if let Some(settings) = &config.settings {
            if let Some(script) = &settings.script {
                let interpreter = settings.interpreter.as_deref().unwrap_or("bash"); //get the parsed interpreter or simply set it to bash

                let script_path = theme_dir.join(script);

                theme_debug(format!(
                    "Running theme script with `{interpreter}`: {}",
                    script_path.display()
                ));
                Command::new(interpreter)
                    .arg(script_path)
                    .spawn()
                    .expect("Failed to run script"); //if it is failed
            } else {
                //skip if no script variable found in settings category file is available
                theme_debug("No script in config.toml settings; skipping theme script");
            }
        } else {
            //skip if no settings category exists in config file
            theme_debug("No settings table in config.toml; skipping theme script");
        }
    } else {
        //skip if no readable/valid configuration file is available
        theme_debug("No valid config.toml found; skipping theme script");
    }

    //Send the message for selected theme is applied
    theme_debug("Sending desktop notification");
    Command::new("notify-send")
        .args([message])
        .output()
        .expect("failed to execute process");

    // copy the configured theme directories, including nested files like nvim/lua/plugins/*
    for folder in folders {
        let source = paths.themes.join(theme_name).join(folder);
        let target = paths.config.join(folder);

        theme_debug(format!(
            "Checking theme folder `{folder}`: {}",
            source.display()
        ));
        if source.exists() {
            theme_debug(format!("Copying `{folder}` into {}", target.display()));
            match copy_theme_path(&source, &target) {
                Ok(_) => theme_debug(format!("Applied `{folder}`")),
                Err(e) => eprintln!("[theme-switcher] Copy error in {folder}: {e}"),
            }
        } else {
            theme_debug(format!("No theme directory found for `{folder}`; skipping"));
        }
    }

    let custom_css_source = theme_dir.join("custom.css");
    let custom_css_target = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/custom.css");
    theme_debug(format!(
        "Checking custom CSS: {}",
        custom_css_source.display()
    ));
    if custom_css_source.exists() {
        theme_debug(format!(
            "Copying custom CSS into {}",
            custom_css_target.display()
        ));
        match copy_theme_path(&custom_css_source, &custom_css_target) {
            Ok(_) => theme_debug("Copied custom.css"),
            Err(e) => eprintln!("[theme-switcher] Copy error in custom.css: {e}"),
        }
    } else {
        theme_debug("No custom.css found for this theme; skipping");
    }

    // Run refresh script for refreshing the system
    theme_debug("Running refresh_system");
    Command::new("refresh_system")
        .spawn()
        .expect("Failed to refresh the system.");

    //get the default wallpaper
    let mut wallpaper = paths.themes.clone();
    wallpaper.push(theme_name);
    wallpaper.push("default.png");

    if wallpaper.exists() {
        theme_debug(format!("Setting wallpaper: {}", wallpaper.display()));

        //use awww to apply the wallpaper
        Command::new("awww")
            .args([
                "img",
                wallpaper.to_str().unwrap(),
                "--transition-type",
                "grow",
                "--transition-duration",
                "1",
            ])
            .spawn()
            .ok();
    } else {
        theme_debug("No default.png found for this theme; skipping wallpaper");
    }

    theme_debug(format!("Finished theme switch for `{display_name}`"));
}

fn apply_gtk_options(paths: &AuroraPaths, config: &Config) {
    let Some(gtk) = &config.gtk else {
        return;
    };

    let gtk_theme_name = gtk
        .theme_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let gtk_icon_theme_name = gtk
        .icon_theme
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    if gtk_theme_name.is_none() && gtk_icon_theme_name.is_none() {
        return;
    }

    for settings_path in [
        paths.config.join("gtk-3.0/settings.ini"),
        paths.config.join("gtk-4.0/settings.ini"),
    ] {
        write_gtk_settings(&settings_path, gtk_theme_name, gtk_icon_theme_name);
    }

    apply_gsettings_theme(gtk_theme_name, gtk_icon_theme_name);
}

fn apply_gsettings_theme(gtk_theme_name: Option<&str>, gtk_icon_theme_name: Option<&str>) {
    if let Some(theme_name) = gtk_theme_name {
        Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.interface",
                "gtk-theme",
                theme_name,
            ])
            .status()
            .expect("Failed to set gtk-theme");
    }

    if let Some(icon_theme_name) = gtk_icon_theme_name {
        Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.interface",
                "icon-theme",
                icon_theme_name,
            ])
            .status()
            .expect("Failed to set icon-theme");
    }
}

fn ensure_ini_setting(lines: &mut Vec<String>, key: &str, value: Option<&str>) {
    let Some(value) = value else {
        return;
    };

    let new_line = format!("{key}={value}");

    if let Some(line) = lines
        .iter_mut()
        .find(|line| line.trim_start().starts_with(&format!("{key}=")))
    {
        *line = new_line;
        return;
    }

    if lines.is_empty() {
        lines.push("[Settings]".to_string());
    } else if !lines.iter().any(|line| line.trim() == "[Settings]") {
        lines.insert(0, "[Settings]".to_string());
    }

    lines.push(new_line);
}

fn write_gtk_settings(
    settings_path: &Path,
    gtk_theme_name: Option<&str>,
    gtk_icon_theme_name: Option<&str>,
) {
    let mut lines: Vec<String> = fs::read_to_string(settings_path)
        .map(|content| content.lines().map(String::from).collect())
        .unwrap_or_else(|_| vec!["[Settings]".to_string()]);

    ensure_ini_setting(&mut lines, "gtk-theme-name", gtk_theme_name);
    ensure_ini_setting(&mut lines, "gtk-icon-theme-name", gtk_icon_theme_name);

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create gtk config directory");
    }

    fs::write(settings_path, lines.join("\n") + "\n").expect("Failed to write gtk settings.ini");
}
fn apply_zed_options(paths: &AuroraPaths, config: &Config) {
    let Some(zed) = &config.zed else {
        return;
    };

    let theme_name = zed.theme_name.trim();

    if theme_name.is_empty() {
        eprintln!("Missing theme_name in [zed]");
        return;
    }

    let settings_path = paths.config.join("zed/settings.json");

    if let Err(error) = write_zed_settings(&settings_path, theme_name) {
        eprintln!("Failed to update Zed settings: {error}");
    }
}
fn write_zed_settings(
    settings_path: &Path,
    theme_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read existing settings if present
    let content = match fs::read_to_string(settings_path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(err) => return Err(err.into()),
    };

    // Parse existing settings
    let mut settings: Value = if content.trim().is_empty() {
        json!({})
    } else {
        json5::from_str(&content)?
    };

    // Ensure root is an object
    if !settings.is_object() {
        return Err("Zed settings root must be a JSON object".into());
    }

    // Update theme
    settings["theme"] = Value::String(theme_name.to_string());

    // Create parent directory if needed
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write back
    fs::write(
        settings_path,
        format!("{}\n", serde_json::to_string_pretty(&settings)?),
    )?;

    println!(
        "Updated Zed theme '{}' in {}",
        theme_name,
        settings_path.display()
    );

    Ok(())
}
fn apply_vscode_options(paths: &AuroraPaths, config: &Config) {
    let Some(vscode) = &config.vscode else {
        return;
    };

    let publisher = vscode.publisher.trim();
    let extension_name = vscode.extension_name.trim();
    let theme_name = vscode.theme_name.trim();

    if publisher.is_empty() || extension_name.is_empty() || theme_name.is_empty() {
        eprintln!(
            "Skipping VS Code theme setup: [vscode] requires publisher, extension_name, and theme_name"
        );
        return;
    }

    install_vscode_extension(&paths.home, publisher, extension_name);

    let settings_path = paths.config.join("Code/User/settings.json");
    if let Err(error) = write_vscode_settings(&settings_path, theme_name) {
        eprintln!("Failed to update VS Code settings: {error}");
    }
}

fn install_vscode_extension(home: &Path, publisher: &str, extension_name: &str) {
    let extension_id = format!("{publisher}.{extension_name}");

    if vscode_extension_is_installed(home, &extension_id) {
        println!("VS Code extension already installed: {extension_id}");
        return;
    }

    match Command::new("code")
        .args(["--install-extension", extension_id.as_str()])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(mut child) => {
            println!("Installing VS Code extension in background: {extension_id}");
            thread::spawn(move || match child.wait() {
                Ok(status) if status.success() => {
                    println!("Installed VS Code extension: {extension_id}");
                }
                Ok(status) => {
                    eprintln!("VS Code extension install failed for {extension_id}: {status}");
                }
                Err(error) => {
                    eprintln!(
                        "Failed to wait for VS Code extension install {extension_id}: {error}"
                    );
                }
            });
        }
        Err(error) => {
            eprintln!("Failed to run VS Code CLI for {extension_id}: {error}");
        }
    }
}

fn vscode_extension_is_installed(home: &Path, extension_id: &str) -> bool {
    let extension_id = extension_id.to_lowercase();
    let extension_prefix = format!("{extension_id}-");

    [
        home.join(".vscode/extensions"),
        home.join(".vscode-insiders/extensions"),
        home.join(".vscode-oss/extensions"),
    ]
    .into_iter()
    .filter_map(|path| fs::read_dir(path).ok())
    .flat_map(|entries| entries.flatten())
    .any(|entry| {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        name == extension_id || name.starts_with(&extension_prefix)
    })
}

fn write_vscode_settings(settings_path: &Path, theme_name: &str) -> std::io::Result<()> {
    let mut settings: Value = fs::read_to_string(settings_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| json!({}));

    settings["workbench.colorTheme"] = Value::String(theme_name.to_string());

    settings["workbench.preferredDarkColorTheme"] = Value::String(theme_name.to_string());

    settings["workbench.preferredLightColorTheme"] = Value::String(theme_name.to_string());

    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(settings_path, serde_json::to_string_pretty(&settings)?)?;

    Ok(())
}
fn copy_theme_path(source: &Path, target: &Path) -> std::io::Result<()> {
    if source.is_file() {
        if should_copy(source, target)? {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(source, target)?;
        }
        return Ok(());
    }

    fs::create_dir_all(target)?;

    let mut pending = vec![(source.to_path_buf(), target.to_path_buf())];

    while let Some((current_source, current_target)) = pending.pop() {
        fs::create_dir_all(&current_target)?;

        for entry in fs::read_dir(current_source)? {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_target = current_target.join(entry.file_name());

            if entry.file_type()?.is_dir() {
                pending.push((entry_path, entry_target));
                continue;
            }

            if should_copy(&entry_path, &entry_target)? {
                if let Some(parent) = entry_target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(entry_path, entry_target)?;
            }
        }
    }

    Ok(())
}

fn should_copy(source: &Path, target: &Path) -> std::io::Result<bool> {
    let source_meta = fs::metadata(source)?;

    let target_meta = match fs::metadata(target) {
        Ok(meta) => meta,
        Err(_) => return Ok(true),
    };

    if !target_meta.is_file() || source_meta.len() != target_meta.len() {
        return Ok(true);
    }

    let source_modified = source_meta.modified()?;
    let target_modified = target_meta.modified()?;

    if source_modified > target_modified {
        return Ok(true);
    }

    Ok(!files_match(source, target)?)
}

fn files_match(source: &Path, target: &Path) -> std::io::Result<bool> {
    const BUFFER_SIZE: usize = 16 * 1024;

    let mut source_file = fs::File::open(source)?;
    let mut target_file = fs::File::open(target)?;
    let mut source_buffer = [0_u8; BUFFER_SIZE];
    let mut target_buffer = [0_u8; BUFFER_SIZE];

    loop {
        let source_read = source_file.read(&mut source_buffer)?;
        let target_read = target_file.read(&mut target_buffer)?;

        if source_read != target_read {
            return Ok(false);
        }

        if source_read == 0 {
            return Ok(true);
        }

        if source_buffer[..source_read] != target_buffer[..target_read] {
            return Ok(false);
        }
    }
}

pub fn waybar_position_change(position: String) -> std::io::Result<()> {
    let paths = aurora_paths();
    let waybar_config = paths.config.join("waybar/config.jsonc");
    let content = fs::read_to_string(&waybar_config)?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Replace the Waybar position line regardless of the current value.
    for line in &mut lines {
        if line.trim_start().starts_with("\"position\":") {
            *line = format!("  \"position\": \"{}\",", position);
        }
    }

    let new_content = lines.join("\n");

    fs::write(waybar_config, new_content)?;

    println!("Updated!");

    Ok(())
}

pub fn hyprland_layout_change(layout: &str) -> std::io::Result<()> {
    let command = format!(r#"hl.config({{ general = {{ layout = "{}" }} }})"#, layout);

    let status = Command::new("hyprctl")
        .args(["eval", command.as_str()])
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("hyprctl exited with status {status}"),
        ));
    }

    Ok(())
}
// load the `style.css` file for gtk apps
pub fn load_css() {
    let provider = CssProvider::new();

    let style_css = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/style.css");
    provider.load_from_path(style_css);

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        eprintln!("Warning: Could not connect to a display.");
    }
}

pub fn download_theme(repo_url: String) {
    let paths = aurora_paths();

    Command::new("git")
        .current_dir(paths.themes)
        .args(["clone", repo_url.as_str()])
        .status()
        .expect("Failed to clone theme");
}

#[derive(Debug, Deserialize, Default)]
pub struct AuroraConfig {
    #[serde(default)]
    pub ghostty: GhosttyConfig,

    #[serde(default)]
    pub settings: SettingsSec,
}

#[derive(Debug, Deserialize, Default)]
pub struct GhosttyConfig {
    #[serde(default)]
    pub blur: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct SettingsSec {
    #[serde(default)]
    pub welcome_app: bool,

    #[serde(default = "default_screensaver")]
    pub screensaver: bool,
}

fn default_screensaver() -> bool {
    true
}

pub fn load_config() -> Result<AuroraConfig, String> {
    let paths = aurora_paths();
    let config_path = paths.config.join("aurora.toml");

    if !config_path.exists() {
        return Err(format!(
            "Aurora configuration file not found: {}",
            config_path.display()
        ));
    }

    let content =
        fs::read_to_string(&config_path).map_err(|e| format!("Failed to read aurora.toml: {e}"))?;

    toml::from_str(&content).map_err(|e| format!("Failed to parse aurora.toml: {e}"))
}

pub fn validate_config(_config: &AuroraConfig) -> Result<(), String> {
    Ok(())
}

pub fn aurora_parse() -> Result<(), String> {
    let paths = aurora_paths();
    let config = load_config()?;

    validate_config(&config)?;

    config.ghostty.apply(&paths)?;
    config.settings.apply(&paths)?;

    Ok(())
}

impl GhosttyConfig {
    pub fn apply(&self, paths: &AuroraPaths) -> Result<(), String> {
        ghostty_blur(paths, self.blur)
    }
}

impl SettingsSec {
    pub fn apply(&self, paths: &AuroraPaths) -> Result<(), String> {
        autostart_welcome_app(paths, self.welcome_app)?;
        hypridle_screensaver(paths, self.screensaver)
    }
}

fn edit_file(
    path: &Path,
    edit: impl FnOnce(&mut Vec<String>) -> Result<(), String>,
) -> Result<(), String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    edit(&mut lines)?;

    fs::write(path, lines.join("\n") + "\n")
        .map_err(|e| format!("Failed to write {}: {e}", path.display()))?;

    Ok(())
}

fn leading_indent(line: &str) -> &str {
    let idx = line
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| i)
        .unwrap_or(line.len());

    &line[..idx]
}

fn matches_line(line: &str, target: &str, comment_prefix: &str) -> bool {
    let trimmed = line.trim();

    if trimmed == target {
        return true;
    }

    if let Some(rest) = trimmed.strip_prefix(comment_prefix) {
        return rest.trim_start() == target;
    }

    false
}

fn toggle_line(
    lines: &mut Vec<String>,
    target: &str,
    comment_prefix: &str,
    commented: bool,
) -> Result<(), String> {
    let index = lines
        .iter()
        .position(|line| matches_line(line, target, comment_prefix))
        .ok_or_else(|| format!("Line not found: {target}"))?;

    let indent = leading_indent(&lines[index]);
    let body = if commented {
        format!("{comment_prefix} {target}")
    } else {
        target.to_string()
    };

    lines[index] = format!("{indent}{body}");
    Ok(())
}

fn line_is_enabled(path: &Path, target: &str, comment_prefix: &str) -> Result<bool, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;

    content
        .lines()
        .find(|line| matches_line(line, target, comment_prefix))
        .map(|line| !line.trim_start().starts_with(comment_prefix))
        .ok_or_else(|| format!("Line not found: {target}"))
}

fn ghostty_config_path(paths: &AuroraPaths) -> PathBuf {
    paths.config.join("ghostty/config.ghostty")
}

fn autostart_path(paths: &AuroraPaths) -> PathBuf {
    paths.config.join("hypr/configs/autostart.lua")
}

fn hypridle_path(paths: &AuroraPaths) -> PathBuf {
    paths.config.join("hypr/hypridle.conf")
}

pub fn autostart_welcome_app(paths: &AuroraPaths, enabled: bool) -> Result<(), String> {
    let path = autostart_path(paths);

    edit_file(&path, |lines| {
        toggle_line(lines, r#"hl.exec_cmd("welcome_app")"#, "--", !enabled)
    })
}

pub fn welcome_app_change(enabled: bool) -> Result<(), String> {
    let paths = aurora_paths();
    autostart_welcome_app(&paths, enabled)
}

pub fn welcome_app_is_enabled() -> Result<bool, String> {
    let paths = aurora_paths();
    line_is_enabled(
        &autostart_path(&paths),
        r#"hl.exec_cmd("welcome_app")"#,
        "--",
    )
}

fn comment_line(line: &str, comment_prefix: &str) -> String {
    let indent = leading_indent(line);
    let body = line[indent.len()..].trim_start();

    if body.starts_with(comment_prefix) {
        line.to_string()
    } else {
        format!("{indent}{comment_prefix} {body}")
    }
}

fn uncomment_line(line: &str, comment_prefix: &str) -> String {
    let indent = leading_indent(line);
    let body = line[indent.len()..].trim_start();

    body.strip_prefix(comment_prefix)
        .map(|rest| format!("{indent}{}", rest.trim_start()))
        .unwrap_or_else(|| line.to_string())
}

pub fn hypridle_screensaver(paths: &AuroraPaths, enabled: bool) -> Result<(), String> {
    let path = hypridle_path(paths);

    edit_file(&path, |lines| {
        let timeout_index = lines
            .iter()
            .position(|line| matches_line(line, "on-timeout = aurora-launch-screensaver", "#"))
            .ok_or_else(|| "Screensaver listener not found".to_string())?;

        let start = (0..=timeout_index)
            .rev()
            .find(|index| matches_line(&lines[*index], "listener {", "#"))
            .ok_or_else(|| "Screensaver listener start not found".to_string())?;

        let end = (timeout_index..lines.len())
            .find(|index| matches_line(&lines[*index], "}", "#"))
            .ok_or_else(|| "Screensaver listener end not found".to_string())?;

        for line in &mut lines[start..=end] {
            if line.trim().is_empty() {
                continue;
            }

            *line = if enabled {
                uncomment_line(line, "#")
            } else {
                comment_line(line, "#")
            };
        }

        Ok(())
    })
}

pub fn screensaver_change(enabled: bool) -> Result<(), String> {
    let paths = aurora_paths();
    hypridle_screensaver(&paths, enabled)
}

pub fn screensaver_is_enabled() -> Result<bool, String> {
    let paths = aurora_paths();
    line_is_enabled(
        &hypridle_path(&paths),
        "on-timeout = aurora-launch-screensaver",
        "#",
    )
}

pub fn ghostty_blur(paths: &AuroraPaths, enabled: bool) -> Result<(), String> {
    let path = ghostty_config_path(paths);

    edit_file(&path, |lines| {
        toggle_line(lines, "config-file = ./colors.ghostty", "#", enabled)?;

        toggle_line(lines, "background = 000000", "#", !enabled)?;
        toggle_line(lines, "foreground = ffffff", "#", !enabled)?;
        toggle_line(lines, "background-opacity = 0.2", "#", !enabled)?;

        Ok(())
    })
}

pub fn ghostty_blur_change(enabled: bool) -> Result<(), String> {
    let paths = aurora_paths();
    ghostty_blur(&paths, enabled)
}

pub fn ghostty_blur_is_enabled() -> Result<bool, String> {
    let paths = aurora_paths();
    line_is_enabled(
        &ghostty_config_path(&paths),
        "background-opacity = 0.2",
        "#",
    )
}
// tests
#[cfg(test)]
mod tests {
    use super::{
        Config, copy_theme_path, should_copy, vscode_extension_is_installed, write_vscode_settings,
        write_zed_settings,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn test_dir(name: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("aurora-{name}-{suffix}"))
    }

    #[test]
    fn should_copy_when_target_is_newer_but_contents_differ() {
        let root = test_dir("should-copy");
        fs::create_dir_all(&root).unwrap();

        let source = root.join("source.txt");
        let target = root.join("target.txt");

        fs::write(&source, "theme-a").unwrap();
        thread::sleep(Duration::from_millis(20));
        fs::write(&target, "theme-b").unwrap();

        assert!(should_copy(&source, &target).unwrap());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn copy_theme_path_copies_nested_files() {
        let root = test_dir("nested-copy");
        let source = root.join("source");
        let target = root.join("target");

        fs::create_dir_all(source.join("nvim/lua/plugins")).unwrap();
        fs::write(source.join("nvim/lua/plugins/init.lua"), "return {}").unwrap();

        copy_theme_path(&source, &target).unwrap();

        assert_eq!(
            fs::read_to_string(target.join("nvim/lua/plugins/init.lua")).unwrap(),
            "return {}"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn write_vscode_settings_updates_all_theme_settings() {
        let root = test_dir("vscode-settings-update");
        let settings_path = root.join("Code/User/settings.json");
        let theme_name = "Theme Name From Config";

        fs::create_dir_all(settings_path.parent().unwrap()).unwrap();
        fs::write(
            &settings_path,
            "{\n  \"workbench.colorTheme\": \"Old Theme\",\n  \"workbench.preferredDarkColorTheme\": \"Dracula Theme\",\n  \"workbench.preferredLightColorTheme\": \"Light Theme\",\n  \"editor.fontSize\": 12\n}\n",
        )
        .unwrap();

        write_vscode_settings(&settings_path, theme_name).unwrap();

        assert_eq!(
            fs::read_to_string(&settings_path).unwrap(),
            "{\n  \"workbench.colorTheme\": \"Theme Name From Config\",\n  \"workbench.preferredDarkColorTheme\": \"Theme Name From Config\",\n  \"workbench.preferredLightColorTheme\": \"Theme Name From Config\",\n  \"editor.fontSize\": 12\n}\n"
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn write_vscode_settings_creates_theme_settings() {
        let root = test_dir("vscode-settings-empty");
        let settings_path = root.join("Code/User/settings.json");
        let theme_name_from_config = "Theme Name From Config";

        write_vscode_settings(&settings_path, theme_name_from_config).unwrap();

        assert_eq!(
            fs::read_to_string(&settings_path).unwrap(),
            "{\n  \"workbench.colorTheme\": \"Theme Name From Config\",\n  \"workbench.preferredDarkColorTheme\": \"Theme Name From Config\",\n  \"workbench.preferredLightColorTheme\": \"Theme Name From Config\",\n}\n"
        );

        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn write_zed_settings_creates_theme_setting() {
        let root = test_dir("zed-settings-empty");
        let settings_path = root.join("zed/settings.json");

        write_zed_settings(&settings_path, "Catppuccin Mocha").unwrap();

        assert_eq!(
            fs::read_to_string(&settings_path).unwrap(),
            "{\n  \"theme\": \"Catppuccin Mocha\"\n}\n"
        );
    }
    #[test]
    fn write_zed_settings_updates_theme_setting() {
        let root = test_dir("zed-settings-update");
        let settings_path = root.join("zed/settings.json");

        fs::create_dir_all(settings_path.parent().unwrap()).unwrap();

        fs::write(
            &settings_path,
            "{\n  \"theme\": \"Old Theme\",\n  \"vim_mode\": true\n}\n",
        )
        .unwrap();

        write_zed_settings(&settings_path, "Catppuccin Mocha").unwrap();

        assert_eq!(
            fs::read_to_string(&settings_path).unwrap(),
            "{\n  \"theme\": \"Catppuccin Mocha\",\n  \"vim_mode\": true\n}\n"
        );
    }

    #[test]
    fn vscode_extension_is_installed_checks_local_extension_dirs() {
        let root = test_dir("vscode-extension-installed");
        let extension_dir = root.join(".vscode/extensions/publisher.theme-name-1.0.0");

        fs::create_dir_all(&extension_dir).unwrap();

        assert!(vscode_extension_is_installed(&root, "publisher.theme-name"));
        assert!(!vscode_extension_is_installed(
            &root,
            "publisher.other-theme"
        ));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn bundled_theme_configs_include_vscode_options() {
        let themes_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../themes");

        for entry in fs::read_dir(themes_dir).unwrap() {
            let entry = entry.unwrap();
            let config_path = entry.path().join("config.toml");

            if !config_path.exists() {
                continue;
            }

            let config: Config = toml::from_str(&fs::read_to_string(&config_path).unwrap())
                .unwrap_or_else(|error| panic!("{}: {error}", config_path.display()));
            let vscode = config
                .vscode
                .unwrap_or_else(|| panic!("{} is missing [vscode]", config_path.display()));

            assert!(!vscode.publisher.trim().is_empty());
            assert!(!vscode.extension_name.trim().is_empty());
            assert!(!vscode.theme_name.trim().is_empty());
        }
    }
}
