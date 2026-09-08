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

// ─────────────────────────────────────────────────────────────────────────────
// Aurora Command Palette
// A Raycast/Spotlight style, keyboard-first control centre for the Linux
// desktop.  One key combo, type what you want, press Enter.
//
//   * Searchable providers: apps, themes, aurora commands, system actions,
//     links, scripts, projects, workspaces, windows, files, reminders.
//   * Modes: `>` shell, `?` web search, `/` files, URL detection, fuzzy search.
//   * Keyboard first: ↑/↓ navigate, Enter run, Tab actions, Ctrl+Enter alt.
//   * Theme-aware: uses Aurora's style.css + custom.css colours, applies
//     themes in-process and reloads the CSS instantly.
// ─────────────────────────────────────────────────────────────────────────────

use aurora::{apply_theme, aurora_paths, load_css, theme_entries};
use gtk4::gdk::{Key, ModifierType};
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Label,
    ListBox, ListBoxRow, Orientation, PolicyType, ScrolledWindow, SearchEntry, SelectionMode,
};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::rc::Rc;
use std::thread;
use std::time::{Duration, SystemTime};

// ─── Constants ─────────────────────────────────────────────────────────────

const APP_ID: &str = "com.aurora.command_palette";
const APP_VERSION: &str = "0.1.0";

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 360;
const DEFAULT_MAX_RESULTS: usize = 10;
const HISTORY_LIMIT: usize = 50;
const FAVORITE_LIMIT: usize = 100;

const CAT_APPS: &str = "Apps";
const CAT_THEMES: &str = "Themes";
const CAT_AURORA: &str = "Aurora";
const CAT_SYSTEM: &str = "System";
const CAT_LINKS: &str = "Links";
const CAT_SCRIPTS: &str = "Scripts";
const CAT_PROJECTS: &str = "Projects";
const CAT_WORKSPACES: &str = "Workspaces";
const CAT_WINDOWS: &str = "Windows";
const CAT_FILES: &str = "Files";
const CAT_REMINDERS: &str = "Reminders";
const CAT_SETTINGS: &str = "Settings";
const CAT_FEATURED: &str = "Palette";

const GLYPH_APP: &str = "󰀻";
const GLYPH_THEME: &str = "󰏘";
const GLYPH_LINK: &str = "󰊤";
const GLYPH_SYSTEM: &str = "󰍜";
const GLYPH_AURORA: &str = "󰇙";
const GLYPH_SETTINGS: &str = "󰓅";
const GLYPH_SCRIPT: &str = "󰆍";
const GLYPH_PROJECT: &str = "󰆍";
const GLYPH_TERMINAL: &str = "󰆍";
const GLYPH_WORKSPACE: &str = "󰮄";
const GLYPH_WINDOW: &str = "󰅫";
const GLYPH_FOLDER: &str = "󰉋";
const GLYPH_FILE: &str = "󰈙";
const GLYPH_REMINDER: &str = "󰃰";
const GLYPH_STAR: &str = "★";
const GLYPH_RECENT: &str = "󰋙";
const GLYPH_RELOAD: &str = "󰔄";
const GLYPH_POWER: &str = "󱎷";
const GLYPH_LOCK: &str = "󰌾";
const GLYPH_SUSPEND: &str = "󰆄";
const GLYPH_LOGOUT: &str = "󰁝";
const GLYPH_VOLUME: &str = "󰕾";
const GLYPH_BRIGHTNESS: &str = "";
const GLYPH_BLUETOOTH: &str = "󰂯";
const GLYPH_SCREENSHOT: &str = "󱎀";
const GLYPH_RECORDER: &str = "󱎴";
const GLYPH_CLIPBOARD: &str = "󰂙";
const GLYPH_DISPLAY: &str = "󰹦";
const GLYPH_SHELL: &str = "󰆍";
const GLYPH_SEARCH: &str = "󰀂";
const GLYPH_ACTIVE: &str = "●";
const ACTIVE_COLOR: &str = "#7bd88f";

// ─── Configuration ──────────────────────────────────────────────────────────

#[derive(Clone, Deserialize)]
struct PaletteSection {
    width: Option<u32>,
    height: Option<u32>,
    max_results: Option<usize>,
    opacity: Option<f64>,
    blur: Option<bool>,
    animations: Option<bool>,
    show_icons: Option<bool>,
}

#[derive(Clone, Deserialize)]
struct SearchSection {
    fuzzy: Option<bool>,
    history: Option<bool>,
    favorites: Option<bool>,
    files: Option<bool>,
    file_roots: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct WebSection {
    search_engine: Option<String>,
    engine_url: Option<String>,
}

#[derive(Clone, Deserialize)]
struct ShortcutsSection {
    toggle: Option<String>,
}

#[derive(Clone, Deserialize)]
struct LinkEntry {
    name: String,
    url: String,
    keywords: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct UserCommandEntry {
    name: String,
    description: Option<String>,
    command: String,
    keywords: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct ProjectEntry {
    name: String,
    path: String,
    icon: Option<String>,
    keywords: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct AliasEntry {
    name: String,
    target: String,
}

#[derive(Clone, Deserialize)]
struct PaletteConfig {
    palette: Option<PaletteSection>,
    search: Option<SearchSection>,
    web: Option<WebSection>,
    shortcuts: Option<ShortcutsSection>,
    links: Option<Vec<LinkEntry>>,
    commands: Option<Vec<UserCommandEntry>>,
    projects: Option<Vec<ProjectEntry>>,
    aliases: Option<Vec<AliasEntry>>,
}

#[derive(Clone)]
struct Config {
    width: u32,
    height: u32,
    max_results: usize,
    opacity: f64,
    blur: bool,
    animations: bool,
    show_icons: bool,
    fuzzy: bool,
    history: bool,
    favorites: bool,
    files: bool,
    file_roots: Vec<String>,
    engine_name: String,
    engine_url: String,
    toggle_shortcut: String,
    links: Vec<LinkEntry>,
    commands: Vec<UserCommandEntry>,
    projects: Vec<ProjectEntry>,
    aliases: Vec<AliasEntry>,
}

fn default_config() -> Config {
    Config {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        max_results: DEFAULT_MAX_RESULTS,
        opacity: 0.92,
        blur: true,
        animations: true,
        show_icons: true,
        fuzzy: true,
        history: true,
        favorites: true,
        files: true,
        file_roots: Vec::new(),
        engine_name: String::from("Google"),
        engine_url: String::from("https://www.google.com/search?q="),
        toggle_shortcut: String::from("SUPER+SPACE"),
        links: Vec::new(),
        commands: Vec::new(),
        projects: Vec::new(),
        aliases: Vec::new(),
    }
}

fn engine_url_for(name: &str, configured: &str) -> (String, String) {
    if !configured.trim().is_empty() {
        return (String::from(name), String::from(configured));
    }
    match name.to_lowercase().as_str() {
        "duckduckgo" => (
            String::from("DuckDuckGo"),
            String::from("https://duckduckgo.com/?q="),
        ),
        "bing" => (String::from("Bing"), String::from("https://www.bing.com/search?q=")),
        "youtube" => (
            String::from("YouTube"),
            String::from("https://www.youtube.com/results?search_query="),
        ),
        "github" => (String::from("GitHub"), String::from("https://github.com/search?q=")),
        "wikipedia" => (
            String::from("Wikipedia"),
            String::from("https://en.wikipedia.org/w/index.php?search="),
        ),
        "startpage" => (
            String::from("Startpage"),
            String::from("https://www.startpage.com/sp/search?query="),
        ),
        "ecosia" => (String::from("Ecosia"), String::from("https://www.ecosia.org/search?q=")),
        _ => (String::from("Google"), String::from("https://www.google.com/search?q=")),
    }
}

fn read_config() -> Config {
    let mut cfg = default_config();

    if let Ok(content) = fs::read_to_string(&config_path()) {
        if let Ok(parsed) = toml::from_str::<PaletteConfig>(&content) {
            if let Some(p) = parsed.palette {
                if let Some(v) = p.width {
                    cfg.width = v
                }
                if let Some(v) = p.height {
                    cfg.height = v
                }
                if let Some(v) = p.max_results {
                    cfg.max_results = v
                }
                if let Some(v) = p.opacity {
                    cfg.opacity = v
                }
                if let Some(v) = p.blur {
                    cfg.blur = v
                }
                if let Some(v) = p.animations {
                    cfg.animations = v
                }
                if let Some(v) = p.show_icons {
                    cfg.show_icons = v
                }
            }
            if let Some(s) = parsed.search {
                if let Some(v) = s.fuzzy {
                    cfg.fuzzy = v
                }
                if let Some(v) = s.history {
                    cfg.history = v
                }
                if let Some(v) = s.favorites {
                    cfg.favorites = v
                }
                if let Some(v) = s.files {
                    cfg.files = v
                }
                if let Some(v) = s.file_roots {
                    cfg.file_roots = v
                }
            }
            if let Some(w) = parsed.web {
                if let Some(v) = w.search_engine {
                    cfg.engine_name = v
                }
                if let Some(v) = w.engine_url {
                    cfg.engine_url = v
                }
            }
            if let Some(sh) = parsed.shortcuts {
                if let Some(v) = sh.toggle {
                    cfg.toggle_shortcut = v
                }
            }
            if let Some(l) = parsed.links {
                cfg.links = l
            }
            if let Some(c) = parsed.commands {
                cfg.commands = c
            }
            if let Some(p) = parsed.projects {
                cfg.projects = p
            }
            if let Some(a) = parsed.aliases {
                cfg.aliases = a
            }
        }
    }

    let (name, url) = engine_url_for(&cfg.engine_name, &cfg.engine_url);
    cfg.engine_name = name;
    cfg.engine_url = url;

    if cfg.file_roots.is_empty() {
        let home = aurora_paths().home;
        cfg.file_roots.push(home.join("Documents").display().to_string());
        cfg.file_roots.push(home.join("Downloads").display().to_string());
        cfg.file_roots.push(home.join("Projects").display().to_string());
        cfg.file_roots.push(home.join("Aurora").display().to_string());
        cfg.file_roots.push(home.join(".config").display().to_string());
    }

    cfg
}

fn write_sample_config() {
    let path = config_path();
    if fs::exists(&path).unwrap_or(false) {
        return;
    }

    let mut content = String::new();
    content.push_str(
        "# Aurora Command Palette configuration\n# Full reference: aurorawiki.vercel.app\n\n\
         [palette]\nwidth = 640\nheight = 360\nmax_results = 10\nopacity = 0.92\n\
         blur = true\nanimations = true\nshow_icons = true\n\n\
         [search]\nfuzzy = true\nhistory = true\nfavorites = true\nfiles = true\n\n\
         [web]\nsearch_engine = \"google\"\n\n\
         [shortcuts]\ntoggle = \"SUPER+SPACE\"\n\n",
    );
    content.push_str(
        "[[links]]\nname = \"Aurora GitHub\"\nurl = \"https://github.com/TheAhumMaitra/Aurora\"\n\
         keywords = [\"aurora\", \"github\", \"source\"]\n\n\
         [[commands]]\nname = \"Restart Waybar\"\ndescription = \"Restart the Waybar process\"\n\
         command = \"waybar_refresh\"\nkeywords = [\"waybar\", \"restart\", \"bar\"]\n\n\
         [[projects]]\nname = \"Aurora\"\npath = \"~/Aurora\"\nkeywords = [\"aurora\", \"rice\"]\n\n\
         [[aliases]]\nname = \"wb\"\ntarget = \"waybar restart\"\n",
    );
    let _ = fs::write(&path, &content);
}

// ─── Data model ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct Result {
    title: String,
    subtitle: String,
    glyph: String,
    category: String,
    keywords: Vec<String>,
    priority: i32,
    action: String,
    dangerous: bool,
    active: bool,
    header: bool,
    secondaries: Vec<(String, String)>,
    alt: String,
}

fn result(
    title: &str,
    subtitle: &str,
    glyph: &str,
    category: &str,
    action: &str,
) -> Result {
    Result {
        title: String::from(title),
        subtitle: String::from(subtitle),
        glyph: String::from(glyph),
        category: String::from(category),
        keywords: Vec::new(),
        priority: 50,
        action: String::from(action),
        dangerous: false,
        active: false,
        header: false,
        secondaries: Vec::new(),
        alt: String::new(),
    }
}

fn header_row(title: &str) -> Result {
    let mut r = result(title, "", "", "", "nop:");
    r.header = true;
    r
}

#[derive(Clone)]
struct FileEntry {
    name: String,
    parent: String,
    path: String,
    is_dir: bool,
}

#[derive(Clone)]
struct MonitorInfo {
    name: String,
    resolution: String,
    scale: String,
}

#[derive(Clone)]
struct Confirm {
    title: String,
    message: String,
    descriptor: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PaletteMode {
    Normal,
    Secondary,
    Recent,
    Favorites,
    Control,
    Command,
}

#[derive(Clone)]
struct Palette {
    results: Vec<Result>,
    select_map: Vec<i32>,
    selected: usize,
    mode: PaletteMode,
    query: String,
    confirm_pending: Option<Confirm>,
    show_output: bool,
    history: Vec<(String, String)>,
    favorites: Vec<(String, String)>,
    config: Config,
    index: Vec<Result>,
    active_theme: String,
    monitors: Vec<MonitorInfo>,
    active_workspace: String,
    terminal: String,
    home: PathBuf,
    files: Vec<FileEntry>,
    file_stack: Vec<String>,
    files_done: bool,
    files_started: bool,
    window: ApplicationWindow,
    search: SearchEntry,
    list: ListBox,
    main_box: GtkBox,
    confirm_box: GtkBox,
    output_box: GtkBox,
    status_label: Label,
    detail_label: Label,
    footer_label: Label,
    confirm_title: Label,
    confirm_msg: Label,
    output_label: Label,
}

fn palette_snapshot(p_rc: &Rc<RefCell<Palette>>) -> Palette {
    p_rc.borrow().clone()
}

fn palette_commit(p_rc: &Rc<RefCell<Palette>>, palette: Palette) {
    *p_rc.borrow_mut() = palette;
}

// ─── Paths & simple state persistence ─────────────────────────────────────

fn palette_dir() -> PathBuf {
    let paths = aurora_paths();
    paths.home.join(".local/share/Aurora/palette")
}

fn scripts_dir() -> PathBuf {
    let paths = aurora_paths();
    paths.config.join("aurora/scripts")
}

fn ensure_dirs() {
    fs::create_dir_all(&palette_dir());
    fs::create_dir_all(&scripts_dir());
    let _ = fs::create_dir_all(&aurora_paths().config.join("aurora"));
}

fn history_path() -> PathBuf {
    palette_dir().join("history.txt")
}

fn favorites_path() -> PathBuf {
    palette_dir().join("favorites.txt")
}

fn reminders_path() -> PathBuf {
    palette_dir().join("reminders.txt")
}

fn config_path() -> PathBuf {
    let paths = aurora_paths();
    paths.config.join("aurora/palette.toml")
}

fn load_pairs(path: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(i) = trimmed.find('\t') {
                out.push((String::from(&trimmed[..i]), String::from(&trimmed[i + 1..])));
            }
        }
    }
    out
}

fn save_pairs(path: &Path, pairs: &[(String, String)]) {
    let mut content = String::new();
    for (title, desc) in pairs {
        content.push_str(title);
        content.push('\t');
        content.push_str(desc);
        content.push('\n');
    }
    let _ = fs::write(path, &content);
}

// ─── Small runtime helpers ─────────────────────────────────────────────────

fn command_exists(name: &str) -> bool {
    match Command::new("sh")
        .args(["-c", &format!("command -v {} >/dev/null 2>&1", name)])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn command_output(args: &[&str]) -> String {
    if args.is_empty() {
        return String::new();
    }
    match Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(mut child) => match child.wait_with_output() {
            Ok(result) => String::from_utf8_lossy(&result.stdout).trim().to_string(),
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    }
}

fn now_unix_secs() -> i64 {
    let now = SystemTime::now();
    now.duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

/// `date -d "<spec>" +%s` — GNU date handles "today 19:00", "tomorrow 6",
/// "+30 minutes", "7pm", ...
fn epoch_from_date_spec(spec: &str) -> Option<i64> {
    fn parse_digits(text: &str) -> Option<i64> {
        let mut value: i64 = 0;
        let mut seen = false;
        for c in text.chars() {
            if c >= '0' && c <= '9' {
                value = value * 10 + (c as i64 - '0' as i64);
                seen = true;
            } else {
                return None;
            }
        }
        if seen {
            Some(value)
        } else {
            None
        }
    }

    match Command::new("date")
        .args(["-d", spec, "+%s"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(mut child) => match child.wait_with_output() {
            Ok(result) => parse_digits(String::from_utf8_lossy(&result.stdout).trim()),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn expand_home(path: &str) -> String {
    let home = aurora_paths().home;
    if path.starts_with("~/") {
        home.join(&path[2..]).display().to_string()
    } else if path == "~" {
        home.display().to_string()
    } else {
        String::from(path)
    }
}

fn humanize_name(name: &str) -> String {
    let mut out = String::new();
    let mut cap = true;
    for c in name.chars() {
        match c {
            '-' | '_' | '.' => {
                out.push(' ');
                cap = true
            }
            _ => {
                if cap {
                    out.push_str(&c.to_string().to_uppercase());
                    cap = false;
                } else {
                    out.push_str(&c.to_string());
                }
            }
        }
    }
    out.trim().to_string()
}

fn escape_markup(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push_str(&c.to_string()),
        }
    }
    out
}

fn detect_terminal() -> String {
    std::env::var("TERMINAL").unwrap_or_else(|_| String::from("kitty"))
}

fn base_name(path: &Path) -> String {
    match path.file_name() {
        Some(n) => n.to_string_lossy().to_string(),
        None => String::from("?"),
    }
}

// ─── Fuzzy matching & ranking ──────────────────────────────────────────────

fn token_score(word: &str, token: &str, fuzzy: bool) -> i32 {
    if token == word {
        return 1000;
    }
    if token.starts_with(word) {
        return 600 + (word.len() as i32);
    }
    if token.contains(word) {
        return 350;
    }
    if !fuzzy {
        return 0;
    }

    // ordered-subsequence match: "drac" → "dracula"
    let wb = word.as_bytes();
    let tb = token.as_bytes();
    let mut wi = 0;
    let mut ti = 0;
    while wi < wb.len() && ti < tb.len() {
        if wb[wi] == tb[ti] {
            wi += 1;
        }
        ti += 1;
    }
    if wi == wb.len() {
        150 - ((tb.len() - wb.len()) as i32)
    } else {
        0
    }
}

fn rank_result(
    r: &Result,
    words: &[&str],
    favorites: &[(String, String)],
    history: &[(String, String)],
    fuzzy: bool,
) -> i32 {
    let mut total: i32 = 0;

    for word in words {
        if word.is_empty() {
            continue;
        }
        let mut best = 0;
        for title_token in r.title.to_lowercase().split_whitespace() {
            let s = token_score(word, title_token, fuzzy);
            if s > best {
                best = s;
            }
        }
        if best == 0 {
            for sub_token in r.subtitle.to_lowercase().split_whitespace() {
                let s = token_score(word, sub_token, fuzzy);
                if s > best {
                    best = s;
                }
            }
        }
        if best == 0 {
            for kw in &r.keywords {
                if kw.to_lowercase() == *word {
                    best = 250;
                    break;
                }
                if kw.to_lowercase().contains(word) {
                    best = 200;
                    break;
                }
            }
        }
        total += best;
    }

    if total == 0 {
        return 0;
    }

    for (title, desc) in favorites {
        if *title == r.title && *desc == r.action {
            total += 300;
            break;
        }
    }
    for (i, (title, desc)) in history.iter().enumerate() {
        if *title == r.title && *desc == r.action {
            total += if i < 40 {
                180 - (i as i32)
            } else {
                130
            };
            break;
        }
    }

    total + r.priority
}

// ─── Query analysis ────────────────────────────────────────────────────────

fn looks_like_url(q: &str) -> bool {
    let lower = q.to_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("www.") {
        return true;
    }
    if lower.starts_with("localhost") && lower.contains(":") {
        return true;
    }
    if lower.contains(' ') {
        return false;
    }
    // bare domain: youtube.com, github.com …
    let chars: Vec<char> = lower.chars().collect();
    let mut saw_dot = false;
    for (idx, c) in chars.iter().enumerate() {
        if *c == '.' && idx > 0 && idx + 1 < chars.len() {
            saw_dot = true;
        }
    }
    saw_dot && chars.len() >= 4
}

fn normalize_url(q: &str) -> String {
    if q.starts_with("http://") || q.starts_with("https://") {
        return String::from(q);
    }
    format!("https://{}", q)
}

/// "remind me to X at 7pm" → (task, date-spec)
fn parse_reminder(q: &str) -> Option<(String, String)> {
    let mut s = q.trim().to_lowercase().to_string();
    if !s.starts_with("remind") {
        return None;
    }
    for prefix in ["remind me to ", "remind me ", "remind "] {
        if s.starts_with(prefix) {
            s = String::from(s[prefix.len()..].trim());
            break;
        }
    }
    if s.is_empty() {
        return None;
    }

    let markers = [(" tomorrow at ", "tomorrow "), (" at ", "today "), (" in ", "+")];
    for (marker, prefix) in markers {
        if let Some(i) = s.find(marker) {
            let task = s[..i].trim();
            if task.is_empty() {
                return None;
            }
            let mut when = String::from(s[i + marker.len()..].trim());
            while when.ends_with('.') || when.ends_with('!') {
                when = String::from(&when[..when.len() - 1]);
            }
            if when.is_empty() {
                return None;
            }
            let spec = if when == "noon" {
                String::from("today 12:00")
            } else if when == "midnight" {
                String::from("today 00:00")
            } else {
                format!("{}{}", prefix, when)
            };
            return Some((String::from(task), spec));
        }
    }
    None
}

/// "volume 70" / "brightness 45" → a control result.
fn parse_level_command(q: &str) -> Option<Result> {
    let words: Vec<String> = q
        .trim()
        .to_lowercase()
        .split_whitespace()
        .map(|w| String::from(w))
        .collect();
    if words.len() != 2 {
        return None;
    }
    let mut value: i64 = 0;
    let mut seen = false;
    for c in words[1].chars() {
        if c >= '0' && c <= '9' {
            value = value * 10 + (c as i64 - '0' as i64);
            seen = true;
        } else if c != '%' {
            return None;
        }
    }
    if !seen {
        return None;
    }
    let pct: u32 = if value > 100 {
        100
    } else {
        value as u32
    };

    match words[0].as_str() {
        "volume" | "vol" | "volu" | "v" => {
            let mut r = result(
                &format!("Set Volume to {}%", pct),
                "Set the default audio sink volume",
                GLYPH_VOLUME,
                CAT_SYSTEM,
                "",
            );
            if command_exists("wpctl") {
                r.action = format!("shell:wpctl set-volume @DEFAULT_AUDIO_SINK@ {}%", pct);
            } else {
                r.action = format!("shell:pactl set-sink-volume @DEFAULT_SINK@ {}%", pct);
            }
            r.keywords = Vec::new();
            r.keywords.push(String::from("volume"));
            r.keywords.push(String::from("sound"));
            r.keywords.push(String::from("audio"));
            r.keywords.push(String::from("level"));
            r.priority = 900;
            Some(r)
        }
        "brightness" | "bright" | "br" | "bri" => {
            let mut r = result(
                &format!("Set Brightness to {}%", pct),
                "Set the display backlight level",
                GLYPH_BRIGHTNESS,
                CAT_SYSTEM,
                &format!("shell:brightnessctl set {}%", pct),
            );
            r.keywords = Vec::new();
            r.keywords.push(String::from("brightness"));
            r.keywords.push(String::from("screen"));
            r.keywords.push(String::from("backlight"));
            r.keywords.push(String::from("light"));
            r.priority = 900;
            Some(r)
        }
        _ => None,
    }
}

/// Web search via a short prefix (`yt linux rice`, `gh aurora`, …).
fn web_prefix_search(q: &str, engine_url: &str, engine_name: &str) -> Option<Result> {
    let lower = q.trim().to_lowercase();

    let mut prefix: Option<(&str, &str)> = None;
    let engines = [
        ("yt ", "https://www.youtube.com/results?search_query="),
        ("gh ", "https://github.com/search?q="),
        ("wiki ", "https://en.wikipedia.org/w/index.php?search="),
        ("ddg ", "https://duckduckgo.com/?q="),
        ("g ", engine_url),
    ];
    for (prefix_, engine) in engines {
        if lower.starts_with(prefix_) {
            prefix = Some((prefix_, engine));
            break;
        }
    }

    let (prefix_str, url) = prefix?;
    let query = q.trim().get(prefix_str.len()..).unwrap_or("").trim();
    if query.is_empty() {
        return None;
    }

    let engine_display = if url.contains("youtube.com") {
        String::from("YouTube")
    } else if url.contains("github.com") {
        String::from("GitHub")
    } else if url.contains("wikipedia.org") {
        String::from("Wikipedia")
    } else if url.contains("duckduckgo.com") {
        String::from("DuckDuckGo")
    } else {
        String::from(engine_name)
    };

    let encoded = urlencoding::encode(query);
    let target = format!("{}{}", url, encoded);

    let mut r = result(
        &format!("Search {} for \"{}\"", engine_display, query),
        &target,
        GLYPH_SEARCH,
        CAT_LINKS,
        &format!("open:{}", target),
    );
    r.keywords.push(String::from("search"));
    r.keywords.push(String::from("web"));
    r.keywords.push(String::from(prefix_str.trim()));
    r.keywords.push(String::from(query));
    r.priority = 950;
    Some(r)
}

// ─── Providers ─────────────────────────────────────────────────────────────

fn set_keywords(r: &mut Result, words: &[&str]) {
    r.keywords = words.iter().map(|w| String::from(*w)).collect();
}

fn push_keywords(r: &mut Result, words: Vec<String>) {
    for w in words {
        r.keywords.push(w);
    }
}

fn aurora_results() -> Vec<Result> {
    let mut out = Vec::new();

    let mut list = result("Aurora Themes", "aurora list-themes", GLYPH_THEME, CAT_AURORA, "term:aurora list-themes");
    set_keywords(&mut list, &["aurora", "theme", "list", "show"]);
    list.priority = 120;
    out.push(list);

    let mut reload = result("Reload Aurora", "aurora reload", GLYPH_RELOAD, CAT_AURORA, "shell:aurora reload");
    set_keywords(&mut reload, &["aurora", "reload", "refresh", "restart"]);
    reload.priority = 160;
    out.push(reload);

    let mut refresh = result("Refresh System", "aurora refresh", GLYPH_SYSTEM, CAT_AURORA, "shell:aurora refresh");
    set_keywords(&mut refresh, &["aurora", "refresh", "system", "waybar"]);
    refresh.priority = 150;
    out.push(refresh);

    let mut info = result("Aurora Information", "aurora information", GLYPH_AURORA, CAT_AURORA, "term:aurora information");
    set_keywords(&mut info, &["aurora", "info", "version", "about"]);
    out.push(info);

    let mut update = result("Update Themes", "aurora update-themes", GLYPH_RELOAD, CAT_AURORA, "term:aurora update-themes");
    set_keywords(&mut update, &["aurora", "update", "themes", "download"]);
    out.push(update);

    let mut ghostty_on = result("Ghostty Theme On", "aurora ghostty theme on", GLYPH_TERMINAL, CAT_AURORA, "shell:aurora ghostty theme on");
    set_keywords(&mut ghostty_on, &["ghostty", "theme", "on", "terminal"]);
    out.push(ghostty_on);

    let mut ghostty_off = result("Ghostty Theme Off", "aurora ghostty theme off", GLYPH_TERMINAL, CAT_AURORA, "shell:aurora ghostty theme off");
    set_keywords(&mut ghostty_off, &["ghostty", "theme", "off", "terminal"]);
    out.push(ghostty_off);

    let mut kitty_on = result("Kitty Theme On", "aurora kitty theme on", GLYPH_TERMINAL, CAT_AURORA, "shell:aurora kitty theme on");
    set_keywords(&mut kitty_on, &["kitty", "theme", "on", "terminal"]);
    out.push(kitty_on);

    let mut kitty_off = result("Kitty Theme Off", "aurora kitty theme off", GLYPH_TERMINAL, CAT_AURORA, "shell:aurora kitty theme off");
    set_keywords(&mut kitty_off, &["kitty", "theme", "off", "terminal"]);
    out.push(kitty_off);

    let mut welcome = result("Welcome App On", "aurora settings welcome-app on", GLYPH_SETTINGS, CAT_AURORA, "shell:aurora settings welcome-app on");
    set_keywords(&mut welcome, &["welcome", "app", "autostart", "on"]);
    out.push(welcome);

    let mut screensaver = result("Screensaver On", "aurora settings screensaver on", GLYPH_SCREENSHOT, CAT_AURORA, "shell:aurora settings screensaver on");
    set_keywords(&mut screensaver, &["screensaver", "idle", "hypridle", "on"]);
    out.push(screensaver);

    let mut survey = result("Play Horror Survey", "aurora game horror survey", GLYPH_SYSTEM, CAT_AURORA, "term:aurora game horror survey");
    set_keywords(&mut survey, &["game", "horror", "survey", "fun", "play"]);
    out.push(survey);

    out
}

fn read_active_theme() -> String {
    let paths = aurora_paths();
    let log = paths.home.join(".local/share/Aurora/theme_name.log");
    fs::read_to_string(&log).map(|c| c.trim().to_string()).unwrap_or_default()
}

fn theme_results(p: &Palette) -> Vec<Result> {
    let mut out = Vec::new();
    let entries = theme_entries();
    let home = aurora_paths().home;

    for entry in entries {
        let dir = entry.directory_name.clone();
        let is_active = dir == p.active_theme;
        let theme_root = home.join(".config/themes").join(&dir);
        let preview = theme_root.join("preview.png").display().to_string();
        let config_file = theme_root.join("config.toml").display().to_string();

        let mut apply = result(
            &format!("Apply {}", entry.display_name),
            &format!("aurora apply-theme {}", dir),
            GLYPH_THEME,
            CAT_THEMES,
            &format!("theme:{}", dir),
        );
        set_keywords(&mut apply, &[&dir, &entry.display_name.to_lowercase(), "theme", "apply", "switch", "aurora"]);
        apply.active = is_active;
        apply.priority = 400;
        apply.alt = format!("open:{}", preview);
        apply.secondaries.push((String::from("Apply Theme"), apply.action.clone()));
        apply.secondaries.push((String::from("Preview Theme"), format!("open:{}", preview)));
        apply.secondaries.push((String::from("Set as Default"), format!("shell:aurora apply-theme {}", dir)));
        apply.secondaries.push((String::from("Open Theme Folder"), format!("open:{}", theme_root.display())));
        apply.secondaries.push((String::from("Edit Theme"), format!("term:nvim {}", config_file)));
        out.push(apply);

        let mut preview_result = result(
            &format!("Preview {}", entry.display_name),
            &preview,
            GLYPH_THEME,
            CAT_THEMES,
            &format!("open:{}", preview),
        );
        set_keywords(&mut preview_result, &[&dir, "theme", "preview", "look"]);
        preview_result.priority = 300;
        out.push(preview_result);

        let mut edit = result(
            &format!("Edit {}", entry.display_name),
            &config_file,
            GLYPH_SCRIPT,
            CAT_THEMES,
            &format!("term:nvim {}", config_file),
        );
        set_keywords(&mut edit, &[&dir, "theme", "edit", "config", "change"]);
        edit.priority = 250;
        out.push(edit);
    }

    out
}

const AURORA_UTILITIES: &[(&str, &str)] = &[
    ("settings", "Open Aurora Settings"),
    ("theme_switcher", "Open Theme Switcher"),
    ("app_entries_home", "Open App Entries Manager"),
    ("keybinds_help", "Open Keybinds Help"),
    ("search", "Open Web Search Popup"),
    ("system_menu", "Open System Menu"),
    ("layout_switcher", "Open Layout Switcher"),
    ("starship_switcher", "Open Starship Prompt Switcher"),
    ("waybar_position_switcher", "Open Waybar Position Switcher"),
    ("waybar_flavour_switcher", "Open Waybar Flavour Switcher"),
    ("rofi_config_switcher", "Open Rofi Config Switcher"),
    ("screenrecorder", "Open Screen Recorder"),
    ("youtube-downloader", "Open YouTube Downloader"),
];

/// A tiny .desktop parser (single-value keys, no localised lookups needed).
fn parse_desktop(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('[') || t.starts_with('#') {
            continue;
        }
        if let Some(i) = t.find('=') {
            let key = t[..i].trim();
            if key.contains('[') {
                continue; // skip localised keys like Name[de]
            }
            if !map.contains_key(key) {
                map.insert(String::from(key), String::from(t[i + 1..].trim()));
            }
        }
    }
    map
}

/// Strip .desktop field codes (%U, %f, …) from an Exec line.
fn clean_exec(exec: &str) -> String {
    let chars: Vec<char> = exec.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' && i + 1 < chars.len() {
            match chars[i + 1] {
                'U' | 'u' | 'F' | 'f' | 'i' | 'c' | 'k' | 'd' | 'D' | 'n' | 'N' | 'v' | 'm' => {
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out.trim().to_string()
}

fn app_results() -> Vec<Result> {
    let mut out = Vec::new();
    let home = aurora_paths().home;

    let mut dirs: Vec<PathBuf> = Vec::new();
    dirs.push(PathBuf::from("/usr/share/applications"));
    dirs.push(home.join(".local/share/applications"));

    for dir in dirs {
        if !fs::exists(&dir).unwrap_or(false) {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let file_name = base_name(&path);
                if !file_name.ends_with(".desktop") {
                    continue;
                }
                let Ok(content) = fs::read_to_string(&path) else {
                    continue;
                };
                let map = parse_desktop(&content);
                if map.get("NoDisplay").map(|v| v == "true").unwrap_or(false)
                    || map.get("Hidden").map(|v| v == "true").unwrap_or(false)
                {
                    continue;
                }
                let Some(app_name) = map.get("Name") else {
                    continue;
                };
                let exec = clean_exec(map.get("Exec").map(|s| s.as_str()).unwrap_or(""));
                if exec.is_empty() {
                    continue;
                }
                let comment = map.get("Comment").map(|s| s.as_str()).unwrap_or("");
                let categories = map.get("Categories").map(|s| s.as_str()).unwrap_or("");
                let keywords = map.get("Keywords").map(|s| s.as_str()).unwrap_or("");
                let terminal = map.get("Terminal").map(|v| v == "true").unwrap_or(false);

                let mut r = result(app_name, comment, GLYPH_APP, CAT_APPS, "");
                if terminal {
                    r.action = format!("term:{}", exec);
                } else {
                    r.action = format!("shell:{}", exec);
                }
                r.alt = format!("term:{}", exec);
                r.priority = 200;
                r.secondaries.push((String::from("Run in Terminal"), r.alt.clone()));

                let mut kws: Vec<String> = Vec::new();
                for w in app_name.to_lowercase().split_whitespace() {
                    kws.push(String::from(w));
                }
                for part in [comment, categories, keywords] {
                    if part.is_empty() {
                        continue;
                    }
                    for w in part.to_lowercase().split_whitespace() {
                        kws.push(String::from(w));
                    }
                }
                r.keywords = kws;
                out.push(r);
            }
        }
    }

    for &(bin, label) in AURORA_UTILITIES {
        if !command_exists(bin) {
            continue;
        }
        let mut r = result(label, &format!("Launch {}", bin), GLYPH_SETTINGS, CAT_APPS, &format!("run:{}", bin));
        r.priority = 180;
        set_keywords(&mut r, &["aurora", label, bin]);
        out.push(r);
    }

    out
}

fn link_results(cfg: &Config) -> Vec<Result> {
    let mut out = Vec::new();

    let defaults: &[(&str, &str, &[&str])] = &[
        ("Open GitHub", "https://github.com", &["git", "hub", "code", "repo"]),
        ("Open YouTube", "https://youtube.com", &["yt", "video", "watch"]),
        ("Open Gmail", "https://mail.google.com", &["mail", "email", "inbox", "google"]),
        ("Open Google Drive", "https://drive.google.com", &["drive", "cloud", "files"]),
        ("Open Reddit", "https://reddit.com", &["forum", "subreddit", "news"]),
        ("Open Wikipedia", "https://wikipedia.org", &["wiki", "encyclopedia"]),
        ("Open ChatGPT", "https://chatgpt.com", &["ai", "chat", "gpt", "assistant"]),
        ("Open Discord", "https://discord.com", &["chat", "voice", "gaming"]),
        ("Open Google", "https://google.com", &["search", "web"]),
        ("Open X", "https://x.com", &["twitter", "social"]),
    ];

    for &(title, url, keywords) in defaults {
        let mut r = result(title, url, GLYPH_LINK, CAT_LINKS, &format!("open:{}", url));
        r.priority = 90;
        r.alt = format!("copy:{}", url);
        r.secondaries.push((String::from("Copy URL"), r.alt.clone()));
        r.keywords = keywords.iter().map(|w| String::from(*w)).collect();
        r.keywords.push(String::from(title.to_lowercase()));
        out.push(r);
    }

    for entry in &cfg.links {
        let mut r = result(
            &entry.name,
            &entry.url,
            GLYPH_LINK,
            CAT_LINKS,
            &format!("open:{}", entry.url),
        );
        r.priority = 95;
        r.alt = format!("copy:{}", entry.url);
        r.secondaries.push((String::from("Copy URL"), r.alt.clone()));
        r.keywords.push(entry.name.to_lowercase());
        if let Some(k) = &entry.keywords {
            for w in k {
                r.keywords.push(w.to_lowercase());
            }
        }
        out.push(r);
    }

    out
}

fn system_results(p: &Palette) -> Vec<Result> {
    let mut out = Vec::new();
    let has_wpctl = command_exists("wpctl");

    let mut lock = result("Lock Screen", "hyprlock", GLYPH_LOCK, CAT_SYSTEM, "run:hyprlock");
    set_keywords(&mut lock, &["lock", "screen", "suspend", "hyprlock"]);
    lock.priority = 220;
    out.push(lock);

    let mut logout = result("Log Out", "hyprshutdown -vt 3", GLYPH_LOGOUT, CAT_SYSTEM, "shell:hyprshutdown -vt 3");
    set_keywords(&mut logout, &["logout", "log out", "exit", "session", "quit"]);
    logout.dangerous = true;
    logout.priority = 210;
    out.push(logout);

    let mut suspend = result("Suspend", "systemctl suspend", GLYPH_SUSPEND, CAT_SYSTEM, "shell:systemctl suspend");
    set_keywords(&mut suspend, &["suspend", "sleep", "hibernate", "standby"]);
    suspend.dangerous = true;
    suspend.priority = 210;
    out.push(suspend);

    let mut reboot = result("Reboot", "systemctl reboot", GLYPH_POWER, CAT_SYSTEM, "shell:systemctl reboot");
    set_keywords(&mut reboot, &["reboot", "restart", "power", "computer"]);
    reboot.dangerous = true;
    reboot.priority = 210;
    out.push(reboot);

    let mut poweroff = result("Power Off", "systemctl poweroff", GLYPH_POWER, CAT_SYSTEM, "shell:systemctl poweroff");
    set_keywords(&mut poweroff, &["power", "off", "shutdown", "halt", "turn off"]);
    poweroff.dangerous = true;
    poweroff.priority = 210;
    out.push(poweroff);

    let mut hypr_reload = result("Restart Hyprland", "hyprctl reload", GLYPH_RELOAD, CAT_SYSTEM, "shell:hyprctl reload");
    set_keywords(&mut hypr_reload, &["hyprland", "restart", "reload", "compositor"]);
    hypr_reload.priority = 190;
    out.push(hypr_reload);

    let mut waybar_restart = result("Restart Waybar", "waybar_refresh", GLYPH_SYSTEM, CAT_SYSTEM, "run:waybar_refresh");
    set_keywords(&mut waybar_restart, &["waybar", "restart", "reload", "bar", "topbar"]);
    waybar_restart.priority = 240;
    waybar_restart.alt = String::from("self:restart-waybar");
    waybar_restart.secondaries.push((String::from("Kill & Start Fresh"), String::from("self:restart-waybar")));
    out.push(waybar_restart);

    let mut pipewire = result(
        "Restart PipeWire",
        "systemctl --user restart pipewire",
        GLYPH_VOLUME,
        CAT_SYSTEM,
        "shell:systemctl --user restart pipewire",
    );
    set_keywords(&mut pipewire, &["pipewire", "audio", "sound", "restart", "pulse"]);
    pipewire.priority = 150;
    out.push(pipewire);

    let mut bluetooth = result(
        "Restart Bluetooth",
        "systemctl restart bluetooth",
        GLYPH_BLUETOOTH,
        CAT_SYSTEM,
        "shell:systemctl restart bluetooth",
    );
    set_keywords(&mut bluetooth, &["bluetooth", "restart", "bt", "devices"]);
    bluetooth.priority = 150;
    out.push(bluetooth);

    let mut shot = result("Screenshot (Output)", "hyprshot -m output", GLYPH_SCREENSHOT, CAT_SYSTEM, "shell:hyprshot -m output");
    set_keywords(&mut shot, &["screenshot", "screen", "shot", "capture", "snapshot"]);
    shot.priority = 170;
    out.push(shot);

    let mut region = result("Screenshot (Region)", "hyprshot -m region", GLYPH_SCREENSHOT, CAT_SYSTEM, "shell:hyprshot -m region");
    set_keywords(&mut region, &["screenshot", "region", "area", "selection", "capture"]);
    region.priority = 170;
    out.push(region);

    let mut recorder = result("Screen Recorder", "screenrecorder", GLYPH_RECORDER, CAT_SYSTEM, "run:screenrecorder");
    set_keywords(&mut recorder, &["record", "recorder", "screen", "video"]);
    recorder.priority = 160;
    out.push(recorder);

    let mut clipboard = result("Clear Clipboard", "wl-copy -c", GLYPH_CLIPBOARD, CAT_SYSTEM, "shell:wl-copy -c");
    set_keywords(&mut clipboard, &["clipboard", "copy", "clear", "paste"]);
    clipboard.priority = 140;
    out.push(clipboard);

    let vol_cmd = if has_wpctl {
        "wpctl set-volume @DEFAULT_AUDIO_SINK@"
    } else {
        "pactl set-sink-volume @DEFAULT_SINK@"
    };
    let mut vol_up = result(
        "Increase Volume",
        &format!("{} 5%+", vol_cmd),
        GLYPH_VOLUME,
        CAT_SYSTEM,
        &format!("shell:{} 5%+", vol_cmd),
    );
    set_keywords(&mut vol_up, &["volume", "sound", "audio", "increase", "up", "louder"]);
    vol_up.priority = 220;
    out.push(vol_up);

    let mut vol_down = result(
        "Decrease Volume",
        &format!("{} 5%-", vol_cmd),
        GLYPH_VOLUME,
        CAT_SYSTEM,
        &format!("shell:{} 5%-", vol_cmd),
    );
    set_keywords(&mut vol_down, &["volume", "sound", "audio", "decrease", "down", "quieter"]);
    vol_down.priority = 220;
    out.push(vol_down);

    let mute_cmd = if has_wpctl {
        "wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle"
    } else {
        "pactl set-sink-mute @DEFAULT_SINK@ toggle"
    };
    let mut mute = result("Mute / Unmute", mute_cmd, GLYPH_VOLUME, CAT_SYSTEM, &format!("shell:{}", mute_cmd));
    set_keywords(&mut mute, &["mute", "unmute", "volume", "sound", "silence"]);
    mute.priority = 220;
    out.push(mute);

    let mut audio_settings = result("Open Audio Settings", "pavucontrol", GLYPH_VOLUME, CAT_SYSTEM, "run:pavucontrol");
    set_keywords(&mut audio_settings, &["audio", "settings", "volume", "sound", "pulse", "mixer"]);
    audio_settings.priority = 130;
    out.push(audio_settings);

    let mut br_up = result(
        "Increase Brightness",
        "brightnessctl set +5%",
        GLYPH_BRIGHTNESS,
        CAT_SYSTEM,
        "shell:brightnessctl set +5%",
    );
    set_keywords(&mut br_up, &["brightness", "screen", "backlight", "increase", "up"]);
    br_up.priority = 220;
    out.push(br_up);

    let mut br_down = result(
        "Decrease Brightness",
        "brightnessctl set 5%-",
        GLYPH_BRIGHTNESS,
        CAT_SYSTEM,
        "shell:brightnessctl set 5%-",
    );
    set_keywords(&mut br_down, &["brightness", "screen", "backlight", "decrease", "down"]);
    br_down.priority = 220;
    out.push(br_down);

    if p.monitors.len() >= 2 {
        let primary = &p.monitors[0];
        let secondary = &p.monitors[1];

        let mut mirror = result(
            "Mirror Displays",
            "Show the same output on every monitor",
            GLYPH_DISPLAY,
            CAT_SYSTEM,
            &format!(
                "shell:hyprctl keyword monitor {},disable && hyprctl keyword monitor {},{},auto,1",
                secondary.name,
                primary.name,
                primary.resolution,
            ),
        );
        set_keywords(&mut mirror, &["display", "monitor", "mirror", "screen"]);
        mirror.priority = 140;
        out.push(mirror);

        let mut extend = result(
            "Extend Displays",
            "Use every connected monitor side by side",
            GLYPH_DISPLAY,
            CAT_SYSTEM,
            &format!(
                "shell:hyprctl keyword monitor {},preferred,auto,1 && hyprctl keyword monitor {},preferred,auto,1",
                primary.name,
                secondary.name,
            ),
        );
        set_keywords(&mut extend, &["display", "monitor", "extend", "multi", "dual"]);
        extend.priority = 140;
        out.push(extend);

        let mut laptop_only = result(
            &format!("{} Only", primary.name),
            &format!("Disable {} and focus {}", secondary.name, primary.name),
            GLYPH_DISPLAY,
            CAT_SYSTEM,
            &format!("shell:hyprctl keyword monitor {},disable", secondary.name),
        );
        set_keywords(&mut laptop_only, &["display", "monitor", "laptop", "only"]);
        laptop_only.priority = 140;
        out.push(laptop_only);

        let mut external_only = result(
            &format!("{} Only", secondary.name),
            &format!("Disable {} and focus {}", primary.name, secondary.name),
            GLYPH_DISPLAY,
            CAT_SYSTEM,
            &format!(
                "shell:hyprctl keyword monitor {},disable && hyprctl keyword monitor {},preferred,0x0,1",
                primary.name,
                secondary.name,
            ),
        );
        set_keywords(&mut external_only, &["display", "monitor", "external", "hdmi", "only"]);
        external_only.priority = 140;
        out.push(external_only);
    }

    let mut display_info = result(
        "Monitor Info",
        "hyprctl monitors",
        GLYPH_DISPLAY,
        CAT_SYSTEM,
        "term:hyprctl monitors",
    );
    set_keywords(&mut display_info, &["monitor", "display", "info", "screen", "resolution"]);
    display_info.priority = 120;
    out.push(display_info);

    out
}

/// Query Hyprland once for monitor and active-workspace state.
fn detect_system_state() -> (Vec<MonitorInfo>, String) {
    let mut monitors = Vec::new();
    let mut active = String::new();

    let monitors_text = command_output(&["hyprctl", "monitors"]);
    let mut current: Option<MonitorInfo> = None;
    for line in monitors_text.lines() {
        let t = line.trim();
        if t.starts_with("Monitor ") && t.contains('(') {
            if let Some(m) = current {
                monitors.push(m);
            }
            let name = String::from(t[8..].split(' ').next().unwrap_or(""));
            current = Some(MonitorInfo {
                name,
                resolution: String::new(),
                scale: String::from("1"),
            });
        } else if let Some(ref mut m) = current {
            if t.contains('@') && t.contains("at ") {
                m.resolution = String::from(t.split(' ').next().unwrap_or("").split('@').next().unwrap_or(""));
            } else if t.starts_with("scale:") {
                m.scale = String::from(t[6..].trim());
            } else if t.starts_with("active workspace:") {
                active = String::from(t[17..].trim().split(' ').next().unwrap_or(""));
            }
        }
    }
    if let Some(m) = current {
        monitors.push(m);
    }

    if active.is_empty() {
        let ws = command_output(&["hyprctl", "activeworkspace"]);
        for line in ws.lines() {
            let t = line.trim();
            if t.starts_with("workspace ID ") {
                active = String::from(t[13..].split(' ').next().unwrap_or(""));
                break;
            }
        }
    }

    (monitors, active)
}

fn script_results() -> Vec<Result> {
    let mut out = Vec::new();
    let base = scripts_dir();
    if !fs::exists(&base).unwrap_or(false) {
        return out;
    }
    if let Ok(entries) = fs::read_dir(&base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let script_name = base_name(&path);
            let mut r = result(
                &humanize_name(&script_name),
                &format!("Aurora script • {}", path.display()),
                GLYPH_SCRIPT,
                CAT_SCRIPTS,
                &format!("term:{}", path.display()),
            );
            set_keywords(&mut r, &["script", &script_name, "run"]);
            r.priority = 170;
            r.secondaries.push((String::from("Run in Background"), format!("shell:{}", path.display())));
            r.secondaries.push((String::from("Edit Script"), format!("term:nvim {}", path.display())));
            out.push(r);
        }
    }
    out
}

fn project_results(cfg: &Config) -> Vec<Result> {
    let mut out = Vec::new();
    let mut seen: Vec<String> = Vec::new();

    for entry in &cfg.projects {
        let path = expand_home(&entry.path);
        let mut open = result(
            &format!("Open {} Project", entry.name),
            &path,
            GLYPH_PROJECT,
            CAT_PROJECTS,
            &format!("open:{}", path),
        );
        open.priority = 200;
        let mut kws: Vec<String> = Vec::new();
        kws.push(entry.name.to_lowercase());
        kws.push(String::from("project"));
        kws.push(String::from("open"));
        if let Some(k) = &entry.keywords {
            for w in k {
                kws.push(w.to_lowercase());
            }
        }
        open.keywords = kws;
        open.secondaries.push((String::from("Open in File Manager"), format!("open:{}", path)));
        open.secondaries.push((String::from("Open Terminal Here"), format!("termdir:{}", path)));
        open.secondaries.push((String::from("Git Status"), format!("term:git -C {} status", path)));
        open.secondaries.push((String::from("Git Log"), format!("term:git -C {} log --oneline -20", path)));
        out.push(open);
        seen.push(path.clone());

        out.push(terminal_in_project(&entry.name, &path));
        out.push(git_status_result(&entry.name, &path));
    }

    let roots = ["Projects", "Developer", "dev", "github", "git", "Code", "Aurora"];
    let home = aurora_paths().home;
    for root in roots {
        let dir = home.join(root);
        if !fs::exists(&dir).unwrap_or(false) {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let display = path.display().to_string();
                if seen.contains(&display) {
                    continue;
                }
                if !fs::exists(&path.join(".git")).unwrap_or(false) {
                    continue;
                }
                let name = base_name(&path);
                let human = humanize_name(&name);

                let mut open = result(
                    &format!("Open {} Project", human),
                    &display,
                    GLYPH_PROJECT,
                    CAT_PROJECTS,
                    &format!("open:{}", display),
                );
                open.priority = 190;
                set_keywords(&mut open, &[&name.to_lowercase(), "project", "open", "folder", "repo"]);
                open.secondaries.push((String::from("Open in File Manager"), format!("open:{}", display)));
                open.secondaries.push((String::from("Edit in Editor"), format!("shell:code {}", display)));
                out.push(open);
                seen.push(display.clone());

                out.push(terminal_in_project(&human, &display));
                out.push(git_status_result(&human, &display));

                let remote = git_remote_url(&display);
                if !remote.is_empty() {
                    let mut gh = result(
                        &format!("Open {} on GitHub", human),
                        &remote,
                        GLYPH_LINK,
                        CAT_PROJECTS,
                        &format!("open:{}", remote),
                    );
                    set_keywords(&mut gh, &["github", &name.to_lowercase(), "repo", "git"]);
                    gh.priority = 170;
                    out.push(gh);
                }
            }
        }
    }
    out
}

fn terminal_in_project(name: &str, path: &str) -> Result {
    let mut r = result(
        &format!("Terminal in {}", name),
        path,
        GLYPH_TERMINAL,
        CAT_PROJECTS,
        &format!("termdir:{}", path),
    );
    set_keywords(&mut r, &[name, "terminal", "project", "open"]);
    r.priority = 170;
    r
}

fn git_status_result(name: &str, path: &str) -> Result {
    let mut r = result(
        &format!("Git Status ({})", name),
        &format!("git -C {} status", path),
        GLYPH_SCRIPT,
        CAT_PROJECTS,
        &format!("term:git -C {} status", path),
    );
    set_keywords(&mut r, &[name, "git", "status", "project", "repo"]);
    r.priority = 160;
    r.secondaries.push((String::from("Git Pull"), format!("term:git -C {} pull", path)));
    r.secondaries.push((String::from("Git Push"), format!("term:git -C {} push", path)));
    r.secondaries.push((String::from("Git Log"), format!("term:git -C {} log --oneline -20", path)));
    r
}

fn git_remote_url(path: &str) -> String {
    let config_file = Path::new(path).join(".git/config");
    if let Ok(content) = fs::read_to_string(&config_file) {
        for line in content.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("url = ") {
                let raw = rest.trim();
                if raw.contains("github.com") || raw.contains("gitlab.com") || raw.contains("bitbucket.org") {
                    let clean = raw
                        .replace("git@", "")
                        .replace("https://", "")
                        .replace("ssh://git@", "")
                        .replace(':', "/");
                    return format!("https://{}", clean.replace(".git", ""));
                }
                return String::from(raw);
            }
        }
    }
    String::new()
}

fn workspace_results(p: &Palette) -> Vec<Result> {
    let mut out = Vec::new();

    let mut move_next = result(
        "Move Window to Next Workspace",
        "hyprctl dispatch workspace r+1",
        GLYPH_WORKSPACE,
        CAT_WORKSPACES,
        "hypr:workspace r+1",
    );
    set_keywords(&mut move_next, &["workspace", "next", "window", "move"]);
    out.push(move_next);

    let mut move_prev = result(
        "Move Window to Previous Workspace",
        "hyprctl dispatch workspace r-1",
        GLYPH_WORKSPACE,
        CAT_WORKSPACES,
        "hypr:workspace r-1",
    );
    set_keywords(&mut move_prev, &["workspace", "previous", "window", "move"]);
    out.push(move_prev);

    let mut to_special = result(
        "Move Window to Scratchpad",
        "hyprctl dispatch movetoworkspacesilent special",
        GLYPH_WORKSPACE,
        CAT_WORKSPACES,
        "hypr:movetoworkspacesilent special",
    );
    set_keywords(&mut to_special, &["scratchpad", "special", "window", "move"]);
    out.push(to_special);

    for n in 1..=10 {
        let active = p.active_workspace == format!("{n}");
        let subtitle = if active {
            String::from("Currently active")
        } else {
            format!("hyprctl dispatch workspace {n}")
        };
        let mut r = result(
            &format!("Workspace {n}"),
            &subtitle,
            GLYPH_WORKSPACE,
            CAT_WORKSPACES,
            &format!("hypr:workspace {n}"),
        );
        r.active = active;
        r.priority = 90;
        set_keywords(&mut r, &["workspace", "switch", "go"]);
        r.keywords.push(format!("{n}"));
        out.push(r);

        let mut move_to = result(
            &format!("Move Window to Workspace {n}"),
            &format!("hyprctl dispatch movetoworkspace {n}"),
            GLYPH_WINDOW,
            CAT_WORKSPACES,
            &format!("hypr:movetoworkspace {n}"),
        );
        move_to.priority = 85;
        set_keywords(&mut move_to, &["workspace", "window", "move"]);
        move_to.keywords.push(format!("{n}"));
        out.push(move_to);
    }

    out
}

fn window_results() -> Vec<Result> {
    let mut out = Vec::new();

    let mut actions: Vec<(String, String, String)> = Vec::new();
    actions.push((String::from("Close Window"), String::from("hypr:killactive"), String::from("kill the focused window")));
    actions.push((String::from("Toggle Fullscreen"), String::from("hypr:fullscreen 1"), String::from("fullscreen the focused window")));
    actions.push((String::from("Toggle Floating"), String::from("hypr:togglefloating"), String::from("float the focused window")));
    actions.push((String::from("Pin Window"), String::from("hypr:pin"), String::from("pin the focused window")));
    actions.push((String::from("Center Window"), String::from("hypr:centerwindow"), String::from("center the focused window")));
    actions.push((String::from("Focus Next Window"), String::from("hypr:cyclenext"), String::from("cycle focus forward")));
    actions.push((String::from("Focus Previous Window"), String::from("hypr:cycleprev"), String::from("cycle focus backward")));

    for (title, action, subtitle) in actions {
        let mut r = result(&title, &subtitle, GLYPH_WINDOW, CAT_WINDOWS, &action);
        set_keywords(&mut r, &["window", "hyprland", "focus", "close"]);
        r.keywords.push(title.to_lowercase());
        r.priority = 140;
        out.push(r);
    }

    let mut resize = result(
        "Resize Window",
        "Interactively resize the focused window",
        GLYPH_WINDOW,
        CAT_WINDOWS,
        "hypr:resizeactive 20 20",
    );
    set_keywords(&mut resize, &["resize", "window", "size", "grow"]);
    resize.priority = 120;
    out.push(resize);

    out
}

fn user_command_results(cfg: &Config) -> Vec<Result> {
    let mut out = Vec::new();
    for entry in &cfg.commands {
        let desc = entry.description.clone().unwrap_or_else(|| entry.command.clone());
        let mut r = result(&entry.name, &desc, GLYPH_SCRIPT, CAT_FEATURED, "");
        r.action = format!("shell:{}", entry.command);
        r.priority = 210;
        let mut kws: Vec<String> = Vec::new();
        kws.push(entry.name.to_lowercase());
        kws.push(desc.to_lowercase());
        if let Some(k) = &entry.keywords {
            for w in k {
                kws.push(w.to_lowercase());
            }
        }
        r.keywords = kws;
        out.push(r);
    }
    out
}

/// Build the full static index once at startup.
fn build_index(p: &Palette) -> Vec<Result> {
    let mut index = Vec::new();
    index.extend(theme_results(p));
    index.extend(aurora_results());
    index.extend(system_results(p));
    index.extend(window_results());
    index.extend(workspace_results(p));
    index.extend(link_results(&p.config));
    index.extend(script_results());
    index.extend(project_results(&p.config));
    index.extend(user_command_results(&p.config));
    index.extend(app_results());
    index
}

// ─── Search pipeline ───────────────────────────────────────────────────────

fn empty_query_results(p: &mut Palette) -> Vec<Result> {
    let mut out = Vec::new();

    if !p.favorites.is_empty() {
        out.push(header_row("Favorites"));
        for (title, desc) in p.favorites.clone() {
            let mut r = result(&title, "Favorite action", GLYPH_STAR, CAT_FEATURED, &desc);
            r.priority = 500;
            r.secondaries.push((
                String::from("Remove from Favorites"),
                format!("unfav:x\t{}\t{}", title, desc),
            ));
            out.push(r);
        }
    }

    if !p.history.is_empty() {
        out.push(header_row("Recent"));
        let count = p.history.len().min(6);
        for (title, desc) in p.history[..count].to_vec() {
            let mut r = result(&title, "Recently used", GLYPH_RECENT, CAT_FEATURED, &desc);
            r.priority = 450;
            out.push(r);
        }
    }

    out.push(header_row("Suggested"));
    let mut settings = result("Open Aurora Settings", "settings", GLYPH_SETTINGS, CAT_SETTINGS, "run:settings");
    settings.priority = 420;
    out.push(settings);
    let mut reminders = result(
        "Today's Reminders",
        "command_palette --list-reminders",
        GLYPH_REMINDER,
        CAT_REMINDERS,
        "self:list-reminders",
    );
    reminders.priority = 400;
    out.push(reminders);
    let mut theme = result("Browse Themes", "theme_switcher", GLYPH_THEME, CAT_THEMES, "run:theme_switcher");
    theme.priority = 400;
    out.push(theme);
    let mut cfg_entry = result(
        "Open Palette Config",
        "palette.toml",
        GLYPH_SETTINGS,
        CAT_FEATURED,
        &format!("term:nvim {}", config_path().display()),
    );
    cfg_entry.priority = 390;
    out.push(cfg_entry);
    if !p.history.is_empty() {
        let mut clear = result("Clear History", "Erase the action history", GLYPH_CLIPBOARD, CAT_FEATURED, "self:clear-history");
        clear.priority = 390;
        out.push(clear);
    }

    out
}

fn control_menu_results() -> Vec<Result> {
    let mut out = Vec::new();
    out.push(header_row("Palette Menu"));
    out.push(result("Clear History", "Erase the action history", GLYPH_CLIPBOARD, CAT_FEATURED, "self:clear-history"));
    out.push(result("Clear Favorites", "Erase all favorites", GLYPH_STAR, CAT_FEATURED, "self:clear-favorites"));
    out.push(result(
        "Open Palette Config",
        "Edit palette.toml",
        GLYPH_SETTINGS,
        CAT_FEATURED,
        &format!("term:nvim {}", config_path().display()),
    ));
    out.push(result(
        "Open Aurora Scripts",
        "Scripts here are indexed automatically",
        GLYPH_SCRIPT,
        CAT_FEATURED,
        &format!("open:{}", scripts_dir().display()),
    ));
    out.push(result(
        &format!("About Aurora Palette {}", APP_VERSION),
        "One key combo, type what you want, press Enter.",
        GLYPH_APP,
        CAT_FEATURED,
        "nop:",
    ));
    out
}

fn compute_results(p: &mut Palette, query: &str) -> Vec<Result> {
    match p.mode {
        PaletteMode::Recent => {
            let mut out = Vec::new();
            if !p.history.is_empty() {
                out.push(header_row("Recent"));
                let count = p.history.len().min(10);
                for (title, desc) in p.history[..count].to_vec() {
                    let mut r = result(&title, "Recently used", GLYPH_RECENT, CAT_FEATURED, &desc);
                    r.priority = 450;
                    out.push(r);
                }
            } else {
                out.push(result("No recent actions yet", "Type something and press Enter", GLYPH_RECENT, CAT_FEATURED, "nop:"));
            }
            out
        }
        PaletteMode::Favorites => {
            let mut out = Vec::new();
            if !p.favorites.is_empty() {
                out.push(header_row("Favorites"));
                for (title, desc) in p.favorites.clone() {
                    let mut r = result(&title, "Favorite action", GLYPH_STAR, CAT_FEATURED, &desc);
                    r.priority = 500;
                    r.secondaries.push((
                        String::from("Remove from Favorites"),
                        format!("unfav:x\t{}\t{}", title, desc),
                    ));
                    out.push(r);
                }
            } else {
                out.push(result("No favorites yet", "Press Tab on any result to favorite it", GLYPH_STAR, CAT_FEATURED, "nop:"));
            }
            out
        }
        PaletteMode::Control => control_menu_results(),
        PaletteMode::Secondary => p.results.clone(),
        PaletteMode::Command => Vec::new(),
        PaletteMode::Normal => normal_search_results(p, query),
    }
}

fn file_search_results(p: &Palette, query: &str) -> Vec<Result> {
    let mut out = Vec::new();
    if !p.files_done {
        out.push(result(
            "Indexing files…",
            "File results appear as the index grows",
            GLYPH_FILE,
            CAT_FILES,
            "nop:",
        ));
    }

    let words: Vec<String> = query
        .to_lowercase()
        .split_whitespace()
        .map(|w| String::from(w))
        .collect();
    if words.is_empty() {
        return out;
    }

    let mut scored: Vec<(i32, Result)> = Vec::new();
    for file in &p.files {
        let mut best = 0;
        for w in &words {
            let s = token_score(w, &file.name.to_lowercase(), p.config.fuzzy);
            if s > best {
                best = s;
            }
        }
        if best == 0 && words.len() == 1 {
            for seg in file.parent.to_lowercase().split_whitespace() {
                let s = token_score(&words[0], seg, p.config.fuzzy);
                if s > best {
                    best = s;
                }
            }
        }
        if best > 0 {
            let glyph = if file.is_dir {
                GLYPH_FOLDER
            } else {
                GLYPH_FILE
            };
            let mut r = result(&file.name, &file.parent, glyph, CAT_FILES, &format!("open:{}", file.path));
            r.priority = best / 20;
            if file.is_dir {
                r.secondaries.push((String::from("Open in Terminal"), format!("termdir:{}", file.path)));
            } else {
                r.secondaries.push((String::from("Open Containing Folder"), format!("open:{}", file.parent)));
            }
            r.secondaries.push((String::from("Copy Path"), format!("copy:{}", file.path)));
            scored.push((best + r.priority, r));
        }
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    let count = scored.len().min(p.config.max_results);
    for (_, r) in &scored[..count] {
        out.push(r.clone());
    }
    if out.is_empty() && p.files_done {
        out.push(result(
            &format!("Nothing matching \"{}\" in the file index", query),
            "Try a different name",
            GLYPH_FILE,
            CAT_FILES,
            "nop:",
        ));
    }
    out
}

fn web_search_result(p: &Palette, query: &str) -> Result {
    let encoded = urlencoding::encode(query);
    let target = format!("{}{}", p.config.engine_url, encoded);
    let mut r = result(
        &format!("Search {} for \"{}\"", p.config.engine_name, query),
        &target,
        GLYPH_SEARCH,
        CAT_LINKS,
        &format!("open:{}", target),
    );
    r.priority = 30;
    r.alt = format!("copy:{}", target);
    r
}

fn normal_search_results(p: &mut Palette, query: &str) -> Vec<Result> {
    let q = query.trim();
    let lower = q.to_lowercase();

    // explicit modes --------------------------------------------------------
    if lower.starts_with('>') && q.len() > 1 {
        let cmd = q[1..].trim();
        p.mode = PaletteMode::Command;
        let mut r = result(
            &format!("Run command: {}", cmd),
            "Execute safely through a shell and show the output here",
            GLYPH_SHELL,
            CAT_FEATURED,
            &format!("shell:{}", cmd),
        );
        r.priority = 1000;
        let mut out = Vec::new();
        out.push(r);
        return out;
    }
    if lower.starts_with('/') {
        let file_query = q[1..].trim();
        return file_search_results(p, file_query);
    }
    if lower.starts_with('?') && q.len() > 1 {
        let web = q[1..].trim();
        let mut out = Vec::new();
        out.push(web_search_result(p, web));
        return out;
    }

    // URL / quick web-search / reminder / level shortcuts --------------------
    if looks_like_url(&lower) {
        let target = normalize_url(&lower);
        let mut r = result(
            &format!("Open {}", target),
            "Open in your default browser",
            GLYPH_LINK,
            CAT_LINKS,
            &format!("open:{}", target),
        );
        r.priority = 1000;
        r.alt = format!("copy:{}", target);
        let mut out = Vec::new();
        out.push(r);
        return out;
    }

    if let Some(web) = web_prefix_search(&lower, &p.config.engine_url, &p.config.engine_name) {
        let mut out = Vec::new();
        out.push(web);
        return out;
    }

    if let Some((task, spec)) = parse_reminder(&lower) {
        if let Some(epoch) = epoch_from_date_spec(&spec) {
            let when = command_output(&["date", "-d", &spec, "+%a %d %b %H:%M"]);
            let mut r = result(
                &format!("Create Reminder: {}", task),
                &format!("Notify on {}", when),
                GLYPH_REMINDER,
                CAT_REMINDERS,
                &format!("reminder:{}|{}", epoch, task),
            );
            r.priority = 1000;
            r.keywords.push(String::from("remind"));
            r.keywords.push(String::from("reminder"));
            r.keywords.push(String::from("later"));
            r.keywords.push(String::from("todo"));
            let mut out = Vec::new();
            out.push(r);
            return out;
        }
    }

    if let Some(level) = parse_level_command(&lower) {
        let mut out = Vec::new();
        out.push(level);
        return out;
    }

    if q.is_empty() {
        return empty_query_results(p);
    }

    // aliases: a single-word query may expand, e.g. wb → waybar restart
    let mut words: Vec<String> = lower
        .split_whitespace()
        .map(|w| String::from(w))
        .collect();
    if words.len() == 1 {
        for alias in &p.config.aliases {
            if alias.name.to_lowercase() == words[0] {
                for w in alias.target.split_whitespace() {
                    words.push(String::from(w));
                }
                break;
            }
        }
    }
    let word_refs: Vec<&str> = words.iter().map(|w| w.as_str()).collect();

    let mut scored: Vec<(i32, Result)> = Vec::new();
    for item in &p.index {
        let score = rank_result(item, &word_refs, &p.favorites, &p.history, p.config.fuzzy);
        if score > 0 {
            scored.push((score, item.clone()));
        }
    }

    if p.config.files && !p.files.is_empty() {
        let file_out = file_search_results(p, &lower);
        for r in file_out {
            if !r.title.starts_with("Indexing") && !r.title.starts_with("Nothing matching") {
                scored.push((r.priority * 40 + 5, r));
            }
        }
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    let mut out: Vec<Result> = Vec::new();
    let count = scored.len().min(p.config.max_results);
    for (_, r) in &scored[..count] {
        out.push(r.clone());
    }

    if out.is_empty() {
        out.push(web_search_result(p, &lower));
    }
    out
}

// ─── Execution ─────────────────────────────────────────────────────────────

fn spawn(argv: &[&str]) {
    if argv.is_empty() {
        return;
    }
    if let Err(e) = Command::new(argv[0])
        .args(&argv[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        eprintln!("command_palette: failed to run `{}`: {e}", argv[0]);
    }
}

fn spawn_shell(cmd: &str) {
    if let Err(e) = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        eprintln!("command_palette: failed to run `{cmd}`: {e}");
    }
}

fn spawn_in_terminal(terminal: &str, cmd: &str) {
    spawn(&[terminal, "-e", "sh", "-c", cmd]);
}

fn spawn_terminal_here(terminal: &str, path: &str) {
    spawn(&[terminal, "--working-directory", path]);
}

fn notify(title: &str, body: &str) {
    let _ = Command::new("notify-send")
        .args(["-a", "Aurora"])
        .arg(title)
        .arg(body)
        .spawn();
}

fn copy_to_clipboard(text: &str) {
    if let Ok(mut child) = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        use std::io::Write;
        if let Some(stdin) = child.stdin.as_mut() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }
}

fn restart_waybar_helper() {
    if command_exists("waybar_refresh") {
        spawn(&["waybar_refresh"]);
    } else {
        spawn_shell("pkill waybar; sleep 0.3; waybar &");
    }
}

fn run_descriptor(p: &mut Palette, desc: &str) {
    if desc.is_empty() {
        return;
    }

    if let Some(rest) = desc.strip_prefix("shell:") {
        spawn_shell(rest);
        return;
    }
    if let Some(rest) = desc.strip_prefix("run:") {
        let parts: Vec<&str> = rest.split_whitespace().collect();
        spawn(&parts);
        return;
    }
    if let Some(rest) = desc.strip_prefix("open:") {
        spawn(&["xdg-open", rest]);
        return;
    }
    if let Some(rest) = desc.strip_prefix("termdir:") {
        spawn_terminal_here(&p.terminal, rest);
        return;
    }
    if let Some(rest) = desc.strip_prefix("term:") {
        spawn_in_terminal(&p.terminal, rest);
        return;
    }
    if let Some(rest) = desc.strip_prefix("theme:") {
        let theme = rest.trim();
        apply_theme(theme);
        load_css();
        return;
    }
    if let Some(rest) = desc.strip_prefix("hypr:") {
        let parts: Vec<&str> = rest.split_whitespace().collect();
        let mut argv: Vec<&str> = Vec::new();
        argv.push("hyprctl");
        argv.push("dispatch");
        for part in parts {
            argv.push(part);
        }
        spawn(&argv);
        return;
    }
    if let Some(rest) = desc.strip_prefix("copy:") {
        copy_to_clipboard(rest);
        return;
    }
    if let Some(rest) = desc.strip_prefix("reminder:") {
        if let Some(i) = rest.find('|') {
            let epoch_text = rest[..i].trim();
            let task = rest[i + 1..].trim();
            if let Ok(epoch) = epoch_text.parse::<i64>() {
                if epoch > now_unix_secs() {
                    let mut pairs = load_pairs(&reminders_path());
                    pairs.push((String::from(epoch_text), String::from(task)));
                    save_pairs(&reminders_path(), &pairs);
                    let when = command_output(&["date", "-d", epoch_text, "+%a %d %b %H:%M"]);
                    notify("Reminder set", &format!("{}\n{}", task, when));
                    ensure_reminder_daemon();
                }
            }
        }
        return;
    }
    if let Some(rest) = desc.strip_prefix("self:") {
        match rest.trim() {
            "list-reminders" => list_reminders(),
            "clear-history" => {
                save_pairs(&history_path(), &Vec::new());
                p.history = Vec::new();
                notify("Palette", "History cleared");
            }
            "clear-favorites" => {
                save_pairs(&favorites_path(), &Vec::new());
                p.favorites = Vec::new();
                notify("Palette", "Favorites cleared");
            }
            "restart-waybar" => restart_waybar_helper(),
            _ => {}
        }
        return;
    }
    if desc.starts_with("fav:") || desc.starts_with("unfav:") {
        let is_add = desc.starts_with("fav:");
        let rest = &desc[4..];
        let parts: Vec<&str> = rest.split('\t').collect();
        if parts.len() >= 3 {
            let title = parts[1];
            let action = parts[2];
            let mut favs = p.favorites.clone();
            let idx = favs.iter().position(|(t, a)| t == title && a == action);
            if is_add {
                if idx.is_none() && favs.len() < FAVORITE_LIMIT {
                    favs.push((String::from(title), String::from(action)));
                }
            } else if let Some(i) = idx {
                favs.remove(i);
            }
            p.favorites = favs;
            save_pairs(&favorites_path(), &p.favorites);
        }
        return;
    }
    // nop: and anything unknown → nothing
}

fn run_shell_capture(cmd: &str) -> String {
    match Command::new("sh")
        .args(["-c", cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => match child.wait_with_output() {
            Ok(result) => {
                let mut text = String::from_utf8_lossy(&result.stdout).to_string();
                text.push_str(&String::from_utf8_lossy(&result.stderr));
                text.trim().to_string()
            }
            Err(e) => format!("Failed to run command: {e}"),
        },
        Err(e) => format!("Failed to run command: {e}"),
    }
}

fn record_history(p: &mut Palette, title: &str, desc: &str) {
    if !p.config.history || desc.starts_with("nop:") || desc.starts_with("fav:") || desc.starts_with("unfav:") {
        return;
    }
    if let Some(i) = p.history.iter().position(|(t, a)| t == title && a == desc) {
        p.history.remove(i);
    }
    p.history.push((String::from(title), String::from(desc)));
    while p.history.len() > HISTORY_LIMIT {
        p.history.remove(0);
    }
    save_pairs(&history_path(), &p.history);
}

// ─── Reminders ─────────────────────────────────────────────────────────────

fn reminder_pairs() -> Vec<(String, String)> {
    load_pairs(&reminders_path())
}

fn list_reminders() {
    let mut pairs = reminder_pairs();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    if pairs.is_empty() {
        println!("No upcoming reminders.");
        return;
    }
    println!("Upcoming reminders:");
    for (epoch, text) in pairs {
        let when = command_output(&["date", "-d", &epoch, "+%a %d %b %H:%M"]);
        println!("  • {} – {}", when, text);
    }
}

/// Fire every reminder that is due (or overdue); used at palette startup and
/// periodically by the background reminder daemon.
fn fire_due_reminders() {
    let now = now_unix_secs();
    let pairs = reminder_pairs();
    let mut due: Vec<(String, String)> = Vec::new();
    let mut kept: Vec<(String, String)> = Vec::new();
    for (epoch, text) in pairs {
        if epoch.parse::<i64>().unwrap_or(0) <= now {
            due.push((epoch, text));
        } else {
            kept.push((epoch, text));
        }
    }
    if !due.is_empty() {
        save_pairs(&reminders_path(), &kept);
    }
    for (_, text) in due {
        notify("Reminder", &text);
    }
}

fn reminder_daemon_loop() {
    loop {
        thread::sleep(Duration::from_secs(20));
        fire_due_reminders();
    }
}

fn ensure_reminder_daemon() {
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("command_palette"));
    let probe = command_output(&["pgrep", "-f", "command_palette --reminder-daemon"]);
    if probe.trim().is_empty() {
        let _ = Command::new(&exe)
            .arg("--reminder-daemon")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    }
}

// ─── File index (stepped from the glib main loop, so the UI never freezes) ─

fn step_file_index(p: &mut Palette) -> bool {
    if p.files_done {
        return true;
    }
    if !p.files_started {
        p.files_started = true;
        for root in &p.config.file_roots {
            p.file_stack.push(expand_home(root));
        }
    }

    let mut budget = 120;
    let max_entries: usize = 4000;
    while budget > 0 && !p.file_stack.is_empty() && p.files.len() < max_entries {
        let dir = p.file_stack.pop().unwrap_or_default();
        let path = Path::new(&dir);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                let is_dir = entry_path.is_dir();
                let name = base_name(&entry_path);
                if name.starts_with('.') {
                    continue;
                }
                let parent = match entry_path.parent() {
                    Some(parent) => parent.display().to_string(),
                    None => dir.clone(),
                };
                p.files.push(FileEntry {
                    name,
                    parent,
                    path: entry_path.display().to_string(),
                    is_dir,
                });
                if is_dir && p.files.len() < max_entries {
                    p.file_stack.push(entry_path.display().to_string());
                    budget -= 1;
                }
            }
        }
        budget -= 1;
    }

    if p.file_stack.is_empty() || p.files.len() >= max_entries {
        p.files_done = true;
    }
    p.files_done
}

// ─── GTK UI ────────────────────────────────────────────────────────────────

fn clear_rows(list: &ListBox) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
}

fn rebuild_list(p: &mut Palette) {
    clear_rows(&p.list);
    p.select_map = Vec::new();

    for i in 0..p.results.len() {
        let r = p.results[i].clone();
        let row = ListBoxRow::new();
        row.set_widget_name(&format!("{i}"));

        if r.header {
            row.set_selectable(false);
            row.set_activatable(false);
            let label = Label::builder().label(&r.title).halign(Align::Start).build();
            label.add_css_class("palette-section");
            row.set_child(Some(&label));
        } else {
            row.add_css_class("palette-row");
            p.select_map.push(i as i32);

            let hbox = GtkBox::new(Orientation::Horizontal, 10);
            hbox.set_hexpand(true);

            let glyph = Label::new(Some(&r.glyph));
            glyph.add_css_class("palette-row-glyph");
            glyph.set_size_request(26, -1);

            let copy_box = GtkBox::new(Orientation::Vertical, 1);
            copy_box.set_hexpand(true);

            let mut title_text = escape_markup(&r.title);
            if r.active {
                title_text.push_str(&format!(
                    "  <span foreground=\"{}\">{}</span>",
                    ACTIVE_COLOR, GLYPH_ACTIVE
                ));
            }
            let title = Label::builder().halign(Align::Start).build();
            title.set_markup(&title_text);
            title.add_css_class("palette-row-title");
            title.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            let subtitle = Label::builder().label(&r.subtitle).halign(Align::Start).build();
            subtitle.add_css_class("palette-row-subtitle");
            subtitle.set_ellipsize(gtk4::pango::EllipsizeMode::End);

            copy_box.append(&title);
            if !r.subtitle.is_empty() {
                copy_box.append(&subtitle);
            }

            let category = Label::new(Some(&r.category));
            category.add_css_class("palette-row-category");

            hbox.append(&glyph);
            hbox.append(&copy_box);
            hbox.append(&category);
            row.set_child(Some(&hbox));
        }

        p.list.append(&row);
    }

    p.selected = 0;
    select_position(p, 0);
}

fn select_position(p: &mut Palette, pos: usize) {
    if p.select_map.is_empty() {
        return;
    }
    let capped = pos.min(p.select_map.len() - 1);
    let row_idx = p.select_map[capped];
    if let Some(row) = p.list.row_at_index(row_idx) {
        p.list.unselect_all();
        p.list.select_row(Some(&row));
    }
    p.selected = capped;
}

fn move_selection(p: &mut Palette, delta: i32) {
    let len = p.select_map.len();
    if len == 0 {
        return;
    }
    let mut next = p.selected as i32 + delta;
    if next < 0 {
        next = (len - 1) as i32;
    }
    if next >= len as i32 {
        next = 0;
    }
    select_position(p, next as usize);
}

fn update_hints(p: &mut Palette) {
    if p.confirm_pending.is_some() {
        p.footer_label.set_label("↵ Confirm      ⎋ Cancel");
        p.detail_label.set_label("This action needs your confirmation.");
        p.status_label.set_label("");
        return;
    }

    if p.show_output {
        p.footer_label.set_label("⎋ Close");
        p.detail_label.set_label("Command output");
        p.status_label.set_label("");
        return;
    }

    if p.mode == PaletteMode::Command {
        p.footer_label.set_label("↵ Run in shell      ⎋ Close");
        p.detail_label.set_label("Run a short command; the output is shown below.");
        return;
    }

    if p.results.is_empty() {
        p.footer_label.set_label("↑↓ Navigate      ↵ Open      ⇥ Actions      ⎋ Close");
        p.detail_label.set_label("No results — press Enter to search the web.");
        p.status_label.set_label("");
        return;
    }

    let idx = p.selected.min(p.results.len() - 1);
    let selected = p.results[idx].clone();
    let hint = if selected.dangerous {
        "↵ Confirm"
    } else {
        "↵ Open"
    };
    let footer = if selected.header {
        String::from("↑↓ Navigate      ⎋ Close")
    } else if !selected.secondaries.is_empty() {
        format!("↑↓ Navigate      {hint}      ⇥ Actions      ⎋ Close")
    } else if !selected.alt.is_empty() {
        format!("↑↓ Navigate      {hint}      ⌃↵ Alternative      ⎋ Close")
    } else {
        format!("↑↓ Navigate      {hint}      ⎋ Close")
    };
    p.footer_label.set_label(&footer);

    let mut detail = selected.subtitle.clone();
    if selected.dangerous && !detail.is_empty() {
        detail.push_str("  —  destructive action, will ask first");
    }
    p.detail_label.set_label(&detail);

    p.status_label.set_label(&format!("{} results", p.results.len()));
}

fn refresh(p: &mut Palette) {
    let query = p.query.clone();
    p.results = compute_results(p, &query);
    rebuild_list(p);
    update_hints(p);
}

fn enter_secondary(p: &mut Palette) {
    if p.selected >= p.results.len() {
        return;
    }
    let selected = p.results[p.selected].clone();
    if selected.header || selected.secondaries.is_empty() {
        return;
    }

    let mut list: Vec<Result> = Vec::new();
    list.push(header_row(&format!("{} — Actions", selected.title)));
    list.push(selected.clone());
    for (label, action) in &selected.secondaries {
        let mut r = result(label, &selected.subtitle, GLYPH_SETTINGS, CAT_FEATURED, action);
        r.priority = 300;
        list.push(r);
    }
    if p.config.favorites {
        let is_fav = p
            .favorites
            .iter()
            .any(|(t, a)| *t == selected.title && *a == selected.action);
        let label = if is_fav {
            "Remove from Favorites"
        } else {
            "Add to Favorites"
        };
        let prefix = if is_fav {
            "unfav"
        } else {
            "fav"
        };
        let mut r = result(
            label,
            "Pin this action to the top of the palette",
            GLYPH_STAR,
            CAT_FEATURED,
            &format!("{}:{}\t{}\t{}", prefix, selected.title, selected.title, selected.action),
        );
        r.priority = 390;
        list.push(r);
    }
    p.mode = PaletteMode::Secondary;
    p.results = list;
    rebuild_list(p);
    update_hints(p);
}

fn show_confirm(p: &mut Palette, title: &str, message: &str, descriptor: &str) {
    p.confirm_pending = Some(Confirm {
        title: String::from(title),
        message: String::from(message),
        descriptor: String::from(descriptor),
    });
    p.confirm_title.set_label(&format!("Are you sure?\n\n{}", title));
    p.confirm_msg.set_label(message);
    p.main_box.set_visible(false);
    p.output_box.set_visible(false);
    p.confirm_box.set_visible(true);
    update_hints(p);
}

fn hide_confirm(p: &mut Palette) {
    p.confirm_pending = None;
    p.confirm_box.set_visible(false);
    p.main_box.set_visible(true);
    refresh(p);
}

fn run_command_mode(p: &mut Palette) {
    if !p.query.starts_with('>') {
        return;
    }
    let cmd = String::from(p.query[1..].trim());
    if cmd.is_empty() {
        return;
    }
    let output = run_shell_capture(&cmd);
    p.output_label.set_label(&output);
    p.show_output = true;
    p.main_box.set_visible(false);
    p.confirm_box.set_visible(false);
    p.output_box.set_visible(true);
    update_hints(p);
    let hist_title = format!("> {}", cmd);
    let hist_desc = format!("shell:{}", cmd);
    record_history(p, &hist_title, &hist_desc);
}

fn activate_selected(p: &mut Palette) {
    if p.selected >= p.results.len() || p.confirm_pending.is_some() {
        return;
    }
    let r = p.results[p.selected].clone();
    if r.header {
        return;
    }
    if r.dangerous {
        show_confirm(p, &r.title, &r.subtitle, &r.action);
        return;
    }
    record_history(p, &r.title, &r.action);
    run_descriptor(p, &r.action);
    p.window.close();
}

fn run_alt_action(p: &mut Palette) {
    if p.selected >= p.results.len() || p.confirm_pending.is_some() {
        return;
    }
    let r = p.results[p.selected].clone();
    if r.header {
        return;
    }
    let (title, desc) = if !r.alt.is_empty() {
        (r.title.clone(), r.alt.clone())
    } else if !r.secondaries.is_empty() {
        (r.title.clone(), r.secondaries[0].1.clone())
    } else {
        return;
    };
    record_history(p, &title, &desc);
    run_descriptor(p, &desc);
    p.window.close();
}

fn handle_key(p_rc: &Rc<RefCell<Palette>>, key: Key, mods: ModifierType) -> Propagation {
    let mut p = palette_snapshot(p_rc);
    let ctrl = mods.contains(ModifierType::CONTROL_MASK);

    // confirmation overlay has its own minimal keymap ------------------------
    if p.confirm_pending.is_some() {
        if key == Key::Escape {
            hide_confirm(&mut p);
            palette_commit(p_rc, p);
            return true.into();
        }
        if key == Key::Return {
            let confirm = p.confirm_pending.clone().unwrap();
            p.confirm_pending = None;
            p.confirm_box.set_visible(false);
            p.main_box.set_visible(true);
            record_history(&mut p, &confirm.title, &confirm.descriptor);
            run_descriptor(&mut p, &confirm.descriptor);
            p.window.close();
            palette_commit(p_rc, p);
            return true.into();
        }
        return false.into();
    }

    // command-mode output panel: only Esc closes -----------------------------
    if p.show_output {
        if key == Key::Escape {
            p.window.close();
            palette_commit(p_rc, p);
            return true.into();
        }
        return false.into();
    }

    if key == Key::Escape {
        match p.mode {
            PaletteMode::Secondary => {
                p.mode = PaletteMode::Normal;
                refresh(&mut p);
                palette_commit(p_rc, p);
                return true.into();
            }
            PaletteMode::Recent | PaletteMode::Favorites | PaletteMode::Control => {
                p.mode = PaletteMode::Normal;
                p.query = p.search.text().to_string();
                if p.query.is_empty() {
                    p.results = empty_query_results(&mut p);
                    rebuild_list(&mut p);
                    update_hints(&mut p);
                } else {
                    refresh(&mut p);
                }
                palette_commit(p_rc, p);
                return true.into();
            }
            _ => {}
        }
        p.window.close();
        palette_commit(p_rc, p);
        return true.into();
    }

    if ctrl {
        match key {
            Key::k => {
                p.mode = PaletteMode::Control;
                p.results = control_menu_results();
                rebuild_list(&mut p);
                update_hints(&mut p);
                palette_commit(p_rc, p);
                return true.into();
            }
            Key::r => {
                p.mode = PaletteMode::Recent;
                let query = p.query.clone();
                p.results = compute_results(&mut p, &query);
                rebuild_list(&mut p);
                update_hints(&mut p);
                palette_commit(p_rc, p);
                return true.into();
            }
            Key::f => {
                p.mode = PaletteMode::Favorites;
                let query = p.query.clone();
                p.results = compute_results(&mut p, &query);
                rebuild_list(&mut p);
                update_hints(&mut p);
                palette_commit(p_rc, p);
                return true.into();
            }
            _ => {}
        }
    }

    match key {
        Key::Return => {
            if p.mode == PaletteMode::Command || p.query.starts_with('>') {
                run_command_mode(&mut p);
            } else if ctrl {
                run_alt_action(&mut p);
            } else {
                activate_selected(&mut p);
            }
            palette_commit(p_rc, p);
            true.into()
        }
        Key::Up => {
            if p.mode != PaletteMode::Command && p.confirm_pending.is_none() {
                move_selection(&mut p, -1);
                update_hints(&mut p);
                palette_commit(p_rc, p);
                return true.into();
            }
            false.into()
        }
        Key::Down => {
            if p.mode != PaletteMode::Command && p.confirm_pending.is_none() {
                move_selection(&mut p, 1);
                update_hints(&mut p);
                palette_commit(p_rc, p);
                return true.into();
            }
            false.into()
        }
        Key::Tab => {
            if p.mode == PaletteMode::Secondary {
                if mods.contains(ModifierType::SHIFT_MASK) {
                    p.mode = PaletteMode::Normal;
                    refresh(&mut p);
                } else {
                    move_selection(&mut p, 1);
                    update_hints(&mut p);
                }
            } else {
                enter_secondary(&mut p);
            }
            palette_commit(p_rc, p);
            true.into()
        }
        Key::Left | Key::Right => {
            if p.mode == PaletteMode::Secondary || p.select_map.len() > 1 {
                move_selection(&mut p, if key == Key::Left { -1 } else { 1 });
                update_hints(&mut p);
                palette_commit(p_rc, p);
                return true.into();
            }
            false.into()
        }
        _ => false.into(),
    }
}

fn on_query_changed(p_rc: &Rc<RefCell<Palette>>, text: &str) {
    let mut p = palette_snapshot(p_rc);
    p.query = String::from(text);

    match p.mode {
        PaletteMode::Recent | PaletteMode::Favorites | PaletteMode::Control if !text.is_empty() => {
            p.mode = PaletteMode::Normal;
        }
        PaletteMode::Secondary => {
            p.mode = PaletteMode::Normal;
        }
        _ => {}
    }
    refresh(&mut p);
    palette_commit(p_rc, p);
}

fn build_ui(app: &Application) {
    let cfg = read_config();
    write_sample_config();
    load_css();

    let (monitors, active_workspace) = detect_system_state();
    let active_theme = read_active_theme();
    let home = aurora_paths().home;
    let terminal = detect_terminal();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aurora Command Palette")
        .default_width(cfg.width as i32)
        .default_height(cfg.height as i32)
        .decorated(false)
        .resizable(true)
        .build();
    window.add_css_class("palette-window");
    window.set_opacity(cfg.opacity);

    // ── search bar ──────────────────────────────────────────────────────────
    let search_box = GtkBox::new(Orientation::Horizontal, 8);
    search_box.add_css_class("palette-searchbox");
    search_box.set_margin_start(14);
    search_box.set_margin_end(14);
    search_box.set_margin_top(14);

    let glyph = Label::new(Some(GLYPH_SEARCH));
    glyph.add_css_class("palette-glyph");
    glyph.set_size_request(30, -1);

    let search = SearchEntry::builder()
        .placeholder_text("Search commands, apps, files, links…")
        .hexpand(true)
        .build();
    search.add_css_class("palette-entry");
    search_box.append(&glyph);
    search_box.append(&search);

    // ── results list ────────────────────────────────────────────────────────
    let list = ListBox::new();
    list.set_selection_mode(SelectionMode::Single);
    list.add_css_class("palette-list");

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .child(&list)
        .build();

    // ── hints / footer ──────────────────────────────────────────────────────
    let detail_label = Label::builder().label("").halign(Align::Start).build();
    detail_label.add_css_class("palette-detail");
    detail_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    detail_label.set_hexpand(true);

    let status_label = Label::builder().label("").halign(Align::End).build();
    status_label.add_css_class("palette-detail");

    let hints_row = GtkBox::new(Orientation::Horizontal, 12);
    hints_row.set_margin_start(16);
    hints_row.set_margin_end(16);
    hints_row.set_margin_top(2);
    hints_row.append(&detail_label);
    hints_row.append(&status_label);

    let footer_label = Label::builder().label("").halign(Align::Start).build();
    footer_label.add_css_class("palette-footer");
    footer_label.set_margin_start(16);
    footer_label.set_margin_bottom(10);

    let main_box = GtkBox::new(Orientation::Vertical, 4);
    main_box.append(&search_box);
    main_box.append(&scroll);
    main_box.append(&hints_row);
    main_box.append(&footer_label);

    // ── confirmation overlay ────────────────────────────────────────────────
    let confirm_title = Label::builder().label("").halign(Align::Center).wrap(true).build();
    confirm_title.add_css_class("palette-confirm-title");
    let confirm_msg = Label::builder().label("").halign(Align::Center).wrap(true).build();
    confirm_msg.add_css_class("palette-confirm-msg");

    let cancel = Button::with_label("Cancel");
    cancel.add_css_class("palette-confirm-cancel");
    let confirm_btn = Button::with_label("Confirm");
    confirm_btn.add_css_class("palette-confirm-ok");

    let button_row = GtkBox::new(Orientation::Horizontal, 14);
    button_row.set_halign(Align::Center);
    button_row.set_margin_top(14);
    button_row.append(&cancel);
    button_row.append(&confirm_btn);

    let confirm_box = GtkBox::new(Orientation::Vertical, 10);
    confirm_box.set_halign(Align::Center);
    confirm_box.set_valign(Align::Center);
    confirm_box.set_hexpand(true);
    confirm_box.set_vexpand(true);
    confirm_box.add_css_class("palette-confirm");
    confirm_box.append(&confirm_title);
    confirm_box.append(&confirm_msg);
    confirm_box.append(&button_row);
    confirm_box.set_visible(false);

    // ── command output panel ────────────────────────────────────────────────
    let output_label = Label::builder().label("").halign(Align::Start).wrap(true).build();
    output_label.add_css_class("palette-output");
    output_label.set_xalign(0.0);

    let output_box = GtkBox::new(Orientation::Vertical, 8);
    output_box.set_vexpand(true);
    output_box.set_margin_start(14);
    output_box.set_margin_end(14);
    output_box.set_margin_top(10);
    output_box.append(&output_label);
    output_box.set_visible(false);

    let root = GtkBox::new(Orientation::Vertical, 0);
    root.add_css_class("palette-root");
    root.append(&main_box);
    root.append(&confirm_box);
    root.append(&output_box);
    window.set_child(Some(&root));

    // ── initial state ───────────────────────────────────────────────────────
    let mut p = Palette {
        results: Vec::new(),
        select_map: Vec::new(),
        selected: 0,
        mode: PaletteMode::Normal,
        query: String::new(),
        confirm_pending: None,
        show_output: false,
        history: load_pairs(&history_path()),
        favorites: load_pairs(&favorites_path()),
        config: cfg,
        index: Vec::new(),
        active_theme,
        monitors,
        active_workspace,
        terminal,
        home,
        files: Vec::new(),
        file_stack: Vec::new(),
        files_done: false,
        files_started: false,
        window,
        search,
        list,
        main_box,
        confirm_box,
        output_box,
        status_label,
        detail_label,
        footer_label,
        confirm_title,
        confirm_msg,
        output_label,
    };

    p.index = build_index(&p);
    p.results = empty_query_results(&mut p);
    rebuild_list(&mut p);
    update_hints(&mut p);

    let p_rc = Rc::new(RefCell::new(p));

    // signals ----------------------------------------------------------------
    p_rc.borrow().search.connect_search_changed({
        let p_rc = p_rc.clone();
        move |entry| {
            let text = entry.text();
            on_query_changed(&p_rc, text.as_str());
        }
    });

    p_rc.borrow().search.add_controller({
        let p_rc = p_rc.clone();
        let ctrl = EventControllerKey::new();
        ctrl.connect_key_pressed(move |_, key, _, mods| handle_key(&p_rc, key, mods));
        ctrl
    });

    p_rc.borrow().window.add_controller({
        let p_rc = p_rc.clone();
        let ctrl = EventControllerKey::new();
        ctrl.connect_key_pressed(move |_, key, _, mods| handle_key(&p_rc, key, mods));
        ctrl
    });

    // row selection (also mouse clicks) → refresh the hint bar
    p_rc.borrow().list.connect_row_selected({
        let p_rc = p_rc.clone();
        move |_, _row| {
            let mut p = palette_snapshot(&p_rc);
            if let Some(row) = p.list.selected_row() {
                if let Ok(i) = row.widget_name().parse::<usize>() {
                    if let Some(pos) = p.select_map.iter().position(|x| *x == i as i32) {
                        p.selected = pos;
                        update_hints(&mut p);
                    }
                }
            }
            palette_commit(&p_rc, p);
        }
    });

    // double-click activates (mouse users)
    p_rc.borrow().list.connect_row_activated({
        let p_rc = p_rc.clone();
        move |_, _row| {
            let mut p = palette_snapshot(&p_rc);
            activate_selected(&mut p);
            palette_commit(&p_rc, p);
        }
    });

    // confirmation buttons
    cancel.connect_clicked({
        let p_rc = p_rc.clone();
        move |_| {
            let mut p = palette_snapshot(&p_rc);
            hide_confirm(&mut p);
            palette_commit(&p_rc, p);
        }
    });

    confirm_btn.connect_clicked({
        let p_rc = p_rc.clone();
        move |_| {
            let mut p = palette_snapshot(&p_rc);
            if let Some(confirm) = p.confirm_pending.clone() {
                p.confirm_pending = None;
                p.confirm_box.set_visible(false);
                p.main_box.set_visible(true);
                record_history(&mut p, &confirm.title, &confirm.descriptor);
                run_descriptor(&mut p, &confirm.descriptor);
                p.window.close();
            }
            palette_commit(&p_rc, p);
        }
    });

    // background file indexing, stepped so the UI stays responsive
    gtk4::glib::timeout_add_local(Duration::from_millis(16), {
        let p_rc = p_rc.clone();
        move || {
            let mut p = palette_snapshot(&p_rc);
            let done = step_file_index(&mut p);
            palette_commit(&p_rc, p);
            if done {
                glib_control_flow_break()
            } else {
                glib_control_flow_continue()
            }
        }
    });

    // reminders: fire overdue ones and make sure the daemon is running
    fire_due_reminders();
    ensure_reminder_daemon();

    p_rc.borrow().window.present();
    p_rc.borrow().search.grab_focus();
}

fn glib_control_flow_continue() -> gtk4::glib::ControlFlow {
    gtk4::glib::ControlFlow::Continue
}

fn glib_control_flow_break() -> gtk4::glib::ControlFlow {
    gtk4::glib::ControlFlow::Break
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut mode = "gui";
    for a in &args[1..] {
        match a.as_str() {
            "--reminder-daemon" | "reminders-daemon" => mode = "daemon",
            "--list-reminders" | "list-reminders" => mode = "list",
            "--clear-history" | "clear-history" => mode = "clear-history",
            "--clear-favorites" | "clear-favorites" => mode = "clear-favorites",
            _ => {}
        }
    }

    match mode {
        "daemon" => {
            ensure_dirs();
            fire_due_reminders();
            reminder_daemon_loop();
        }
        "list" => {
            ensure_dirs();
            list_reminders();
        }
        "clear-history" => {
            ensure_dirs();
            save_pairs(&history_path(), &Vec::new());
            println!("History cleared.");
        }
        "clear-favorites" => {
            ensure_dirs();
            save_pairs(&favorites_path(), &Vec::new());
            println!("Favorites cleared.");
        }
        _ => {
            ensure_dirs();
            let app = Application::builder().application_id(APP_ID).build();
            app.connect_activate(build_ui);
            app.run();
        }
    }
}
