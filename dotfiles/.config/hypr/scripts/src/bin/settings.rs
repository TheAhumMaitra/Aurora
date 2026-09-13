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

use aurora::load_css;
use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{
    Align, Application, ApplicationWindow, Box as GtkBox, Button, EventControllerKey, Label,
    ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry, SelectionMode,
};
use std::cell::Cell;
use std::fs;
use std::process::Command;
use std::rc::Rc;

struct SettingsCategory {
    title: &'static str,
    description: &'static str,
    items: &'static [SettingsItem],
}

struct SettingsItem {
    title: &'static str,
    description: &'static str,
    button: &'static str,
    keywords: &'static str,
    action: SettingsAction,
}

#[derive(Clone, Copy)]
enum SettingsAction {
    Edit(&'static str),
    Launch(&'static str),
    Run(&'static str, &'static [&'static str]),
    Open(&'static str),
}

const SETTINGS_CATEGORIES: &[SettingsCategory] = &[
    SettingsCategory {
        title: "Appearance",
        description: "Shape the look of Aurora across themes, wallpapers, bars, launchers, and prompts.",
        items: &[
            SettingsItem {
                title: "Themes",
                description: "Preview and apply a complete Aurora theme.",
                button: "Open",
                keywords: "colors gtk hyprland waybar rofi",
                action: SettingsAction::Launch("theme_switcher"),
            },
            SettingsItem {
                title: "Theme wallpapers",
                description: "Choose from wallpapers bundled with the active theme.",
                button: "Open",
                keywords: "background wallpaper waytrogen",
                action: SettingsAction::Launch("waytrogen_line_change_for_theme"),
            },
            SettingsItem {
                title: "Global wallpapers",
                description: "Choose a wallpaper from your global wallpaper collection.",
                button: "Open",
                keywords: "background wallpaper pictures",
                action: SettingsAction::Launch("waytrogen_line_change_for_global_wallpapers"),
            },
            SettingsItem {
                title: "Waybar layout",
                description: "Switch between the installed Waybar layouts.",
                button: "Open",
                keywords: "bar panel floating",
                action: SettingsAction::Launch("waybar_flavour_switcher"),
            },
            SettingsItem {
                title: "Waybar position",
                description: "Move the bar to the top, bottom, left, or right edge.",
                button: "Open",
                keywords: "bar panel top bottom left right",
                action: SettingsAction::Launch("waybar_position_switcher"),
            },
            SettingsItem {
                title: "Rofi style",
                description: "Apply one of the installed launcher configurations.",
                button: "Open",
                keywords: "launcher menu application",
                action: SettingsAction::Launch("rofi_config_switcher"),
            },
            SettingsItem {
                title: "Starship prompt",
                description: "Choose the prompt configuration used by Fish and other shells.",
                button: "Open",
                keywords: "shell terminal prompt fish",
                action: SettingsAction::Launch("starship_switcher"),
            },
            SettingsItem {
                title: "Update themes",
                description: "Download updates for Aurora's installed theme collection.",
                button: "Update",
                keywords: "themes download git network update",
                action: SettingsAction::Run("aurora", &["update-themes"]),
            },
        ],
    },
    SettingsCategory {
        title: "Hyprland",
        description: "Switch layouts, tune the compositor, and edit the files that control your session.",
        items: &[
            SettingsItem {
                title: "Window layout",
                description: "Switch between dwindle, master, scrolling, monocle, and grid.",
                button: "Open",
                keywords: "tiling master dwindle grid scrolling",
                action: SettingsAction::Launch("layout_switcher"),
            },
            SettingsItem {
                title: "Look and feel",
                description: "Edit gaps, borders, rounding, opacity, shadows, and blur.",
                button: "Edit",
                keywords: "gaps border rounding blur shadow opacity",
                action: SettingsAction::Edit(".config/hypr/configs/look_and_feel.lua"),
            },
            SettingsItem {
                title: "Animations",
                description: "Tune Hyprland animation curves, speeds, and styles.",
                button: "Edit",
                keywords: "motion transition animation speed",
                action: SettingsAction::Edit(".config/hypr/configs/animations.lua"),
            },
            SettingsItem {
                title: "Input devices",
                description: "Configure keyboard layouts, mouse sensitivity, and touchpad scrolling.",
                button: "Edit",
                keywords: "keyboard mouse touchpad sensitivity layout",
                action: SettingsAction::Edit(".config/hypr/configs/input.lua"),
            },
            SettingsItem {
                title: "Layout behavior",
                description: "Configure master, dwindle, and scrolling layout behavior.",
                button: "Edit",
                keywords: "master dwindle scrolling columns",
                action: SettingsAction::Edit(".config/hypr/configs/layout_configs.lua"),
            },
            SettingsItem {
                title: "Keybinds",
                description: "Edit Aurora's Super-key shortcuts and layout bindings.",
                button: "Edit",
                keywords: "keyboard shortcuts hotkeys binds",
                action: SettingsAction::Edit(".config/hypr/configs/keybinds.lua"),
            },
            SettingsItem {
                title: "Monitors",
                description: "Edit monitor positions, resolutions, refresh rates, and scale.",
                button: "Edit",
                keywords: "display screen resolution scale output",
                action: SettingsAction::Edit(".config/hypr/monitors.conf"),
            },
            SettingsItem {
                title: "Reload Hyprland",
                description: "Apply compositor configuration changes without logging out.",
                button: "Reload",
                keywords: "refresh apply restart compositor",
                action: SettingsAction::Run("hyprctl", &["reload"]),
            },
            SettingsItem {
                title: "Window rules",
                description: "Control floating, workspace, opacity, and application-specific behavior.",
                button: "Edit",
                keywords: "windows rules floating opacity workspace",
                action: SettingsAction::Edit(".config/hypr/configs/window_rules.lua"),
            },
            SettingsItem {
                title: "Layer rules",
                description: "Tune overlays such as bars, launchers, and notification surfaces.",
                button: "Edit",
                keywords: "layers rules overlay blur opacity",
                action: SettingsAction::Edit(".config/hypr/configs/layer_rules.lua"),
            },
            SettingsItem {
                title: "Autostart",
                description: "Choose the services and helpers started with your Hyprland session.",
                button: "Edit",
                keywords: "startup services dock waybar launch",
                action: SettingsAction::Edit(".config/hypr/configs/autostart.lua"),
            },
            SettingsItem {
                title: "Environment variables",
                description: "Edit session-wide variables passed to Wayland applications.",
                button: "Edit",
                keywords: "environment variables xdg wayland paths",
                action: SettingsAction::Edit(".config/hypr/configs/env_vars.lua"),
            },
            SettingsItem {
                title: "Custom layouts",
                description: "Edit Aurora's additional Hyprland layout definitions.",
                button: "Edit",
                keywords: "layout custom tiling",
                action: SettingsAction::Edit(".config/hypr/configs/custom_layouts.lua"),
            },
        ],
    },
    SettingsCategory {
        title: "Shell and notifications",
        description: "Control Waybar, the command palette, and the SwayNC notification center.",
        items: &[
            SettingsItem {
                title: "Refresh Waybar",
                description: "Restart Waybar after changing its configuration.",
                button: "Refresh",
                keywords: "bar restart reload",
                action: SettingsAction::Launch("waybar_refresh"),
            },
            SettingsItem {
                title: "Toggle Waybar",
                description: "Show or hide the bar for a distraction-free workspace.",
                button: "Toggle",
                keywords: "bar show hide",
                action: SettingsAction::Launch("waybar_toggle"),
            },
            SettingsItem {
                title: "Command palette",
                description: "Open Aurora's keyboard-first launcher and desktop control center.",
                button: "Open",
                keywords: "launcher commands spotlight raycast search",
                action: SettingsAction::Launch("command_palette"),
            },
            SettingsItem {
                title: "Notifications",
                description: "Open the SwayNC notification center.",
                button: "Open",
                keywords: "swaync alerts messages",
                action: SettingsAction::Run("swaync-client", &["-t", "-sw"]),
            },
            SettingsItem {
                title: "Do Not Disturb",
                description: "Toggle notification quiet mode in SwayNC.",
                button: "Toggle",
                keywords: "notifications mute quiet dnd",
                action: SettingsAction::Run("swaync-client", &["-d", "-sw"]),
            },
            SettingsItem {
                title: "Waybar configuration",
                description: "Edit modules, click actions, workspaces, and status widgets.",
                button: "Edit",
                keywords: "bar modules clock battery volume",
                action: SettingsAction::Edit(".config/waybar/config.jsonc"),
            },
            SettingsItem {
                title: "Waybar styling",
                description: "Edit bar spacing, typography, and widget styling.",
                button: "Edit",
                keywords: "bar css colors font",
                action: SettingsAction::Edit(".config/waybar/style.css"),
            },
            SettingsItem {
                title: "Command palette configuration",
                description: "Edit palette size, opacity, search providers, aliases, and custom commands.",
                button: "Edit",
                keywords: "palette aliases commands search opacity",
                action: SettingsAction::Edit(".config/aurora/palette.toml"),
            },
            SettingsItem {
                title: "Notification configuration",
                description: "Edit notification timeouts, widgets, DND, and control-center behavior.",
                button: "Edit",
                keywords: "swaync notifications widgets timeout",
                action: SettingsAction::Edit(".config/swaync/config.json"),
            },
            SettingsItem {
                title: "Notification styling",
                description: "Edit the colors, spacing, and visual treatment of SwayNC.",
                button: "Edit",
                keywords: "swaync css colors styling",
                action: SettingsAction::Edit(".config/swaync/style.css"),
            },
            SettingsItem {
                title: "Waybar colors",
                description: "Edit the theme-linked color tokens used by Waybar.",
                button: "Edit",
                keywords: "waybar colors theme css",
                action: SettingsAction::Edit(".config/waybar/colors.css"),
            },
            SettingsItem {
                title: "Reload Aurora",
                description: "Re-apply Aurora integrations after changing theme-linked files.",
                button: "Reload",
                keywords: "aurora reload theme apply",
                action: SettingsAction::Run("aurora", &["reload"]),
            },
        ],
    },
    SettingsCategory {
        title: "Aurora tools",
        description: "Open the focused utilities included with the Aurora desktop.",
        items: &[
            SettingsItem {
                title: "Keybinds help",
                description: "Browse every Aurora shortcut, including layout-specific bindings.",
                button: "Open",
                keywords: "shortcuts hotkeys keyboard help",
                action: SettingsAction::Launch("keybinds_help"),
            },
            SettingsItem {
                title: "Search",
                description: "Open the lightweight web search popup.",
                button: "Open",
                keywords: "web browser query",
                action: SettingsAction::Launch("search"),
            },
            SettingsItem {
                title: "Application entries",
                description: "Manage Aurora's custom desktop and web application entries.",
                button: "Open",
                keywords: "apps desktop web browser launcher",
                action: SettingsAction::Launch("app_entries_home"),
            },
            SettingsItem {
                title: "Welcome",
                description: "Open the Aurora welcome panel and onboarding shortcuts.",
                button: "Open",
                keywords: "onboarding help documentation",
                action: SettingsAction::Launch("welcome_app"),
            },
            SettingsItem {
                title: "Workspace overview",
                description: "Inspect workspaces and move focus between open windows.",
                button: "Open",
                keywords: "windows workspaces overview",
                action: SettingsAction::Launch("workspace_overview"),
            },
            SettingsItem {
                title: "Focus timer",
                description: "Start a distraction-free Pomodoro-style focus session.",
                button: "Open",
                keywords: "pomodoro timer productivity",
                action: SettingsAction::Launch("focus_timer"),
            },
            SettingsItem {
                title: "Quick note",
                description: "Capture a note without leaving the current workspace.",
                button: "Open",
                keywords: "notes scratchpad text",
                action: SettingsAction::Launch("quick_note"),
            },
            SettingsItem {
                title: "Clock",
                description: "Show a large clock and date overlay.",
                button: "Open",
                keywords: "time date",
                action: SettingsAction::Launch("clock"),
            },
            SettingsItem {
                title: "Start screen recording",
                description: "Record every monitor to ~/Videos/Screencasts.",
                button: "Start",
                keywords: "record video capture wf-recorder",
                action: SettingsAction::Run("screenrecorder", &[]),
            },
            SettingsItem {
                title: "Record a region",
                description: "Select a region with Slurp and record it to ~/Videos/Screencasts.",
                button: "Region",
                keywords: "record video capture wf-recorder slurp",
                action: SettingsAction::Run("screenrecorder", &["region"]),
            },
            SettingsItem {
                title: "Stop screen recording",
                description: "Stop the active recording and notify with its saved path.",
                button: "Stop",
                keywords: "record video capture wf-recorder stop",
                action: SettingsAction::Run("screenrecorder", &["stop"]),
            },
            SettingsItem {
                title: "YouTube Downloader",
                description: "Download a YouTube video into ~/Downloads/YT_vids",
                button: "Open",
                keywords: "youtube video download media",
                action: SettingsAction::Launch("youtube-downloader"),
            },
        ],
    },
    SettingsCategory {
        title: "Session and hardware",
        description: "Lock, suspend, refresh, and access the hardware controls used by Aurora.",
        items: &[
            SettingsItem {
                title: "Refresh system",
                description: "Refresh Waybar and reload Hyprland together.",
                button: "Run",
                keywords: "reload refresh apply",
                action: SettingsAction::Launch("refresh_system"),
            },
            SettingsItem {
                title: "Lock screen",
                description: "Lock the session with the configured Hyprlock screen.",
                button: "Lock",
                keywords: "hyprlock lock password security",
                action: SettingsAction::Launch("hyprlock"),
            },
            SettingsItem {
                title: "Suspend",
                description: "Suspend the system using systemd.",
                button: "Suspend",
                keywords: "sleep power systemd",
                action: SettingsAction::Run("systemctl", &["suspend"]),
            },
            SettingsItem {
                title: "Power menu",
                description: "Open the configured logout, reboot, shutdown, and suspend menu.",
                button: "Open",
                keywords: "logout reboot shutdown power wlogout",
                action: SettingsAction::Launch("wlogout"),
            },
            SettingsItem {
                title: "Screenshot",
                description: "Capture the current output with Hyprshot.",
                button: "Capture",
                keywords: "screen capture image",
                action: SettingsAction::Run("hyprshot", &["-m", "output"]),
            },
            SettingsItem {
                title: "Audio mixer",
                description: "Open the WireMix PipeWire audio mixer.",
                button: "Open",
                keywords: "sound volume pulse pipewire",
                action: SettingsAction::Run("wiremix", &[]),
            },
            SettingsItem {
                title: "Increase brightness",
                description: "Raise the backlight by five percent.",
                button: "+5%",
                keywords: "screen backlight brightness",
                action: SettingsAction::Run("brightnessctl", &["set", "+5%"]),
            },
            SettingsItem {
                title: "Decrease brightness",
                description: "Lower the backlight by five percent.",
                button: "-5%",
                keywords: "screen backlight brightness",
                action: SettingsAction::Run("brightnessctl", &["set", "5%-"]),
            },
            SettingsItem {
                title: "Mute audio",
                description: "Toggle mute on the default PipeWire audio sink.",
                button: "Toggle",
                keywords: "sound volume audio mute",
                action: SettingsAction::Run(
                    "wpctl",
                    &["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"],
                ),
            },
            SettingsItem {
                title: "Idle and lock behavior",
                description: "Edit screensaver, lock, display power, and suspend timeouts.",
                button: "Edit",
                keywords: "hypridle idle lock suspend dpms timeout",
                action: SettingsAction::Edit(".config/hypr/hypridle.conf"),
            },
            SettingsItem {
                title: "Lock screen appearance",
                description: "Edit the Hyprlock wallpaper, clock, labels, and password field.",
                button: "Edit",
                keywords: "hyprlock lock wallpaper clock password",
                action: SettingsAction::Edit(".config/hypr/hyprlock.conf"),
            },
            SettingsItem {
                title: "Logout menu styling",
                description: "Edit the appearance of Aurora's logout and power menu.",
                button: "Edit",
                keywords: "wlogout power menu css colors",
                action: SettingsAction::Edit(".config/wlogout/style.css"),
            },
            SettingsItem {
                title: "Refresh Aurora",
                description: "Refresh Waybar and reload Hyprland using Aurora's standard command.",
                button: "Run",
                keywords: "aurora refresh apply system",
                action: SettingsAction::Run("aurora", &["refresh"]),
            },
        ],
    },
    SettingsCategory {
        title: "Audio and media",
        description: "Control PipeWire, media playback, microphones, and the hardware keys without leaving Aurora.",
        items: &[
            SettingsItem {
                title: "Volume up",
                description: "Raise the default PipeWire sink by five percent and show the OSD.",
                button: "+5%",
                keywords: "audio sound volume pipewire wpctl speaker",
                action: SettingsAction::Run(
                    "wpctl",
                    &["set-volume", "-l", "1", "@DEFAULT_AUDIO_SINK@", "5%+"],
                ),
            },
            SettingsItem {
                title: "Volume down",
                description: "Lower the default PipeWire sink by five percent and show the OSD.",
                button: "-5%",
                keywords: "audio sound volume pipewire wpctl speaker",
                action: SettingsAction::Run(
                    "wpctl",
                    &["set-volume", "@DEFAULT_AUDIO_SINK@", "5%-"],
                ),
            },
            SettingsItem {
                title: "Mute output",
                description: "Toggle the default speaker or headset mute state.",
                button: "Mute",
                keywords: "audio sound volume mute speaker headphones",
                action: SettingsAction::Run(
                    "wpctl",
                    &["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"],
                ),
            },
            SettingsItem {
                title: "Mute microphone",
                description: "Toggle the default microphone mute state.",
                button: "Mute",
                keywords: "audio microphone input mute mic",
                action: SettingsAction::Run(
                    "wpctl",
                    &["set-mute", "@DEFAULT_AUDIO_SOURCE@", "toggle"],
                ),
            },
            SettingsItem {
                title: "Audio mixer",
                description: "Open WireMix to choose devices, profiles, and per-application volumes.",
                button: "Open",
                keywords: "audio mixer pipewire devices wiremix",
                action: SettingsAction::Run("wiremix", &[]),
            },
            SettingsItem {
                title: "Media play / pause",
                description: "Toggle playback in the active MPRIS player.",
                button: "Play",
                keywords: "music media player mpris playerctl",
                action: SettingsAction::Run("playerctl", &["play-pause"]),
            },
            SettingsItem {
                title: "Next track",
                description: "Skip to the next track in the active MPRIS player.",
                button: "Next",
                keywords: "music media player mpris playerctl",
                action: SettingsAction::Run("playerctl", &["next"]),
            },
            SettingsItem {
                title: "Previous track",
                description: "Return to the previous track in the active MPRIS player.",
                button: "Previous",
                keywords: "music media player mpris playerctl",
                action: SettingsAction::Run("playerctl", &["previous"]),
            },
            SettingsItem {
                title: "SwayOSD configuration",
                description: "Edit the on-screen display used for volume and brightness feedback.",
                button: "Edit",
                keywords: "osd volume brightness swayosd",
                action: SettingsAction::Edit(".config/swayosd/style.css"),
            },
        ],
    },
    SettingsCategory {
        title: "Displays and workspaces",
        description: "Jump between workspaces, inspect outputs, and tune multi-monitor behavior in one place.",
        items: &[
            SettingsItem {
                title: "Workspace overview",
                description: "See open windows and move focus between every Hyprland workspace.",
                button: "Open",
                keywords: "hyprland workspace windows overview",
                action: SettingsAction::Launch("workspace_overview"),
            },
            SettingsItem {
                title: "Focus workspace 1",
                description: "Move focus to the first workspace.",
                button: "Go",
                keywords: "workspace desktop monitor hyprland",
                action: SettingsAction::Run("hyprctl", &["dispatch", "workspace", "1"]),
            },
            SettingsItem {
                title: "Focus workspace 2",
                description: "Move focus to the second workspace.",
                button: "Go",
                keywords: "workspace desktop monitor hyprland",
                action: SettingsAction::Run("hyprctl", &["dispatch", "workspace", "2"]),
            },
            SettingsItem {
                title: "Focus workspace 3",
                description: "Move focus to the third workspace.",
                button: "Go",
                keywords: "workspace desktop monitor hyprland",
                action: SettingsAction::Run("hyprctl", &["dispatch", "workspace", "3"]),
            },
            SettingsItem {
                title: "Focus workspace 4",
                description: "Move focus to the fourth workspace.",
                button: "Go",
                keywords: "workspace desktop monitor hyprland",
                action: SettingsAction::Run("hyprctl", &["dispatch", "workspace", "4"]),
            },
            SettingsItem {
                title: "Toggle all displays",
                description: "Toggle DPMS for every output using Hyprland's standard dispatcher.",
                button: "Toggle",
                keywords: "monitor display screen dpms power",
                action: SettingsAction::Run("hyprctl", &["dispatch", "dpms", "toggle"]),
            },
            SettingsItem {
                title: "Increase brightness",
                description: "Raise the backlight by five percent.",
                button: "+5%",
                keywords: "screen backlight brightness display",
                action: SettingsAction::Run("brightnessctl", &["-e4", "-n2", "set", "5%+"]),
            },
            SettingsItem {
                title: "Decrease brightness",
                description: "Lower the backlight by five percent.",
                button: "-5%",
                keywords: "screen backlight brightness display",
                action: SettingsAction::Run("brightnessctl", &["-e4", "-n2", "set", "5%-"]),
            },
            SettingsItem {
                title: "Monitor configuration",
                description: "Edit output positions, resolutions, refresh rates, and scaling.",
                button: "Edit",
                keywords: "monitor display screen resolution refresh scale",
                action: SettingsAction::Edit(".config/hypr/monitors.conf"),
            },
            SettingsItem {
                title: "Workspace rules",
                description: "Edit the Lua workspace map used by Aurora.",
                button: "Edit",
                keywords: "workspace desktop rules lua monitor",
                action: SettingsAction::Edit(".config/hypr/workspaces.lua"),
            },
            SettingsItem {
                title: "Monitor info",
                description: "Open a live Hyprland output report in a terminal.",
                button: "Inspect",
                keywords: "monitor display screen diagnostics hyprctl",
                action: SettingsAction::Run("kitty", &["-e", "hyprctl", "monitors", "all"]),
            },
        ],
    },
    SettingsCategory {
        title: "Services and diagnostics",
        description: "Inspect the session, troubleshoot services, and launch the monitors already used by Aurora.",
        items: &[
            SettingsItem {
                title: "System monitor",
                description: "Open btop with CPU, memory, process, disk, and network telemetry.",
                button: "Open",
                keywords: "processes cpu memory ram system btop",
                action: SettingsAction::Run("kitty", &["--class", "btop", "btop"]),
            },
            SettingsItem {
                title: "Failed user services",
                description: "Inspect failed systemd user units in a terminal.",
                button: "Inspect",
                keywords: "systemd services daemon failed logs",
                action: SettingsAction::Run("kitty", &["-e", "systemctl", "--user", "--failed"]),
            },
            SettingsItem {
                title: "User service journal",
                description: "Open the current user service journal for troubleshooting.",
                button: "Logs",
                keywords: "systemd services journal logs debug",
                action: SettingsAction::Run("kitty", &["-e", "journalctl", "--user", "-e"]),
            },
            SettingsItem {
                title: "Hyprland clients",
                description: "Inspect every window, class, title, workspace, and PID.",
                button: "Inspect",
                keywords: "windows processes clients hyprland diagnostics",
                action: SettingsAction::Run("kitty", &["-e", "hyprctl", "clients"]),
            },
            SettingsItem {
                title: "Hyprland version",
                description: "Show the compositor version and build information.",
                button: "Inspect",
                keywords: "hyprland version diagnostics info",
                action: SettingsAction::Run("kitty", &["-e", "hyprctl", "version"]),
            },
            SettingsItem {
                title: "Wi-Fi TUI",
                description: "Open the network manager interface used by Aurora's Waybar.",
                button: "Open",
                keywords: "wifi network internet nmcli wifitui",
                action: SettingsAction::Run("kitty", &["--class", "wifitui", "wifitui"]),
            },
            SettingsItem {
                title: "Bluetooth TUI",
                description: "Manage Bluetooth devices from Aurora's terminal interface.",
                button: "Open",
                keywords: "bluetooth devices wireless bluetui",
                action: SettingsAction::Run("kitty", &["--class", "bluetui", "bluetui"]),
            },
            SettingsItem {
                title: "System information",
                description: "Open Aurora's fetch configuration and hardware summary.",
                button: "Open",
                keywords: "info hardware neofetch leenfetch system",
                action: SettingsAction::Run("kitty", &["-e", "leenfetch"]),
            },
            SettingsItem {
                title: "Refresh everything",
                description: "Restart Waybar and reload Hyprland using Aurora's standard helper.",
                button: "Refresh",
                keywords: "reload refresh waybar hyprland",
                action: SettingsAction::Launch("refresh_system"),
            },
        ],
    },
    SettingsCategory {
        title: "Profiles and workflows",
        description: "Launch the small, focused tools that make an Aurora session productive.",
        items: &[
            SettingsItem {
                title: "Focus timer",
                description: "Start a Pomodoro-style focus session with task and break presets.",
                button: "Open",
                keywords: "pomodoro productivity timer workflow",
                action: SettingsAction::Launch("focus_timer"),
            },
            SettingsItem {
                title: "Quick note",
                description: "Capture a note and open the notes folder without leaving the desktop.",
                button: "Open",
                keywords: "notes scratchpad workflow text",
                action: SettingsAction::Launch("quick_note"),
            },
            SettingsItem {
                title: "Command palette",
                description: "Search commands, applications, files, and desktop actions.",
                button: "Open",
                keywords: "launcher search commands applications workflow",
                action: SettingsAction::Launch("command_palette"),
            },
            SettingsItem {
                title: "App entries",
                description: "Manage custom desktop and web applications surfaced by Aurora.",
                button: "Open",
                keywords: "applications desktop web workflow launcher",
                action: SettingsAction::Launch("app_entries_home"),
            },
            SettingsItem {
                title: "Screen recording",
                description: "Start a full-output recording using Aurora's wf-recorder helper.",
                button: "Record",
                keywords: "capture video wf-recorder workflow",
                action: SettingsAction::Run("screenrecorder", &[]),
            },
            SettingsItem {
                title: "Screenshot",
                description: "Capture the current output with Hyprshot.",
                button: "Capture",
                keywords: "capture image screenshot workflow",
                action: SettingsAction::Run("hyprshot", &["-m", "output"]),
            },
            SettingsItem {
                title: "Application autostart",
                description: "Edit the programs and services launched with the session.",
                button: "Edit",
                keywords: "startup session workflow applications services",
                action: SettingsAction::Edit(".config/hypr/configs/autostart.lua"),
            },
            SettingsItem {
                title: "Jolt configuration",
                description: "Edit the project and task workflow configuration for Jolt.",
                button: "Edit",
                keywords: "tasks projects workflow jolt",
                action: SettingsAction::Edit(".config/jolt/config.toml"),
            },
        ],
    },
    SettingsCategory {
        title: "Power user",
        description: "Hidden switches and deep shortcuts for tuning Aurora without leaving the desktop.",
        items: &[
            SettingsItem {
                title: "Disable animations",
                description: "Instantly disable Hyprland animations for a snappier session.",
                button: "Disable",
                keywords: "power user animation motion performance speed",
                action: SettingsAction::Run("hyprctl", &["keyword", "animations:enabled", "false"]),
            },
            SettingsItem {
                title: "Disable blur",
                description: "Instantly disable compositor blur when you need maximum performance.",
                button: "Disable",
                keywords: "power user blur effects decoration performance",
                action: SettingsAction::Run(
                    "hyprctl",
                    &["keyword", "decoration:blur:enabled", "false"],
                ),
            },
            SettingsItem {
                title: "Reload Aurora theme",
                description: "Reapply the current theme and refresh its generated integrations.",
                button: "Reload",
                keywords: "power user theme css colors refresh",
                action: SettingsAction::Run("aurora", &["reload"]),
            },
            SettingsItem {
                title: "Inspect active theme",
                description: "Open the active theme manifest and metadata in the terminal.",
                button: "Inspect",
                keywords: "power user theme info metadata manifest",
                action: SettingsAction::Run("kitty", &["-e", "aurora", "theme-info"]),
            },
            SettingsItem {
                title: "Aurora CLI help",
                description: "Discover every available Aurora command and maintenance action.",
                button: "Discover",
                keywords: "power user cli commands help discover",
                action: SettingsAction::Run("kitty", &["-e", "aurora", "--help"]),
            },
            SettingsItem {
                title: "Edit window rules",
                description: "Tune floating, sizing, pinning, and special-window behavior.",
                button: "Edit",
                keywords: "power user hyprland windows floating rules",
                action: SettingsAction::Edit(".config/hypr/configs/window_rules.lua"),
            },
            SettingsItem {
                title: "Edit layer rules",
                description: "Tune overlays, bars, notifications, and layer-shell behavior.",
                button: "Edit",
                keywords: "power user hyprland layers overlays notifications",
                action: SettingsAction::Edit(".config/hypr/configs/layer_rules.lua"),
            },
            SettingsItem {
                title: "Open Aurora data",
                description: "Browse Aurora's local state, history, favorites, and theme records.",
                button: "Open",
                keywords: "power user data state history favorites cache",
                action: SettingsAction::Open(".local/share/Aurora"),
            },
        ],
    },
    SettingsCategory {
        title: "Configuration files",
        description: "Open the most useful Aurora configuration files directly in your editor.",
        items: &[
            SettingsItem {
                title: "Hyprland entrypoint",
                description: "Edit the main Lua configuration and sourced modules.",
                button: "Edit",
                keywords: "hyprland compositor lua",
                action: SettingsAction::Edit(".config/hypr/hyprland.lua"),
            },
            SettingsItem {
                title: "GTK appearance",
                description: "Edit GTK theme, icon theme, cursor, and font preferences.",
                button: "Edit",
                keywords: "gtk icons cursor font dark mode",
                action: SettingsAction::Edit(".config/gtk-4.0/settings.ini"),
            },
            SettingsItem {
                title: "Kitty terminal",
                description: "Edit terminal font, opacity, and theme behavior.",
                button: "Edit",
                keywords: "terminal kitty font opacity",
                action: SettingsAction::Edit(".config/kitty/kitty.conf"),
            },
            SettingsItem {
                title: "Ghostty terminal",
                description: "Edit Ghostty font, opacity, keybinds, and color integration.",
                button: "Edit",
                keywords: "terminal ghostty font opacity",
                action: SettingsAction::Edit(".config/ghostty/config.ghostty"),
            },
            SettingsItem {
                title: "Fish shell",
                description: "Edit shell startup, Starship, and Aurora helper integration.",
                button: "Edit",
                keywords: "shell fish terminal starship",
                action: SettingsAction::Edit(".config/fish/config.fish"),
            },
            SettingsItem {
                title: "Starship configuration",
                description: "Edit prompt modules, symbols, and terminal status styling.",
                button: "Edit",
                keywords: "shell prompt starship terminal",
                action: SettingsAction::Edit(".config/starship.toml"),
            },
            SettingsItem {
                title: "Aurora color palette",
                description: "Edit shared color tokens used by Aurora's generated styles.",
                button: "Edit",
                keywords: "colors palette accent css theme",
                action: SettingsAction::Edit(".config/hypr/colors.lua"),
            },
            SettingsItem {
                title: "Kitty theme colors",
                description: "Edit the active Kitty color integration files.",
                button: "Edit",
                keywords: "kitty terminal colors theme",
                action: SettingsAction::Edit(".config/kitty/colors.conf"),
            },
            SettingsItem {
                title: "Ghostty theme colors",
                description: "Edit the active Ghostty color integration file.",
                button: "Edit",
                keywords: "ghostty terminal colors theme",
                action: SettingsAction::Edit(".config/ghostty/colors.ghostty"),
            },
        ],
    },
    SettingsCategory {
        title: "Folders",
        description: "Browse Aurora's managed configuration and theme directories.",
        items: &[
            SettingsItem {
                title: "Hyprland folder",
                description: "Open ~/.config/hypr in your file manager.",
                button: "Open",
                keywords: "folder files",
                action: SettingsAction::Open(".config/hypr"),
            },
            SettingsItem {
                title: "Themes folder",
                description: "Open ~/.config/themes.",
                button: "Open",
                keywords: "folder themes files",
                action: SettingsAction::Open(".config/themes"),
            },
            SettingsItem {
                title: "Waybar folder",
                description: "Open ~/.config/waybar.",
                button: "Open",
                keywords: "folder bar files",
                action: SettingsAction::Open(".config/waybar"),
            },
            SettingsItem {
                title: "Aurora scripts",
                description: "Open the Rust scripts source folder.",
                button: "Open",
                keywords: "folder rust scripts development",
                action: SettingsAction::Open(".config/hypr/scripts/src/bin"),
            },
            SettingsItem {
                title: "Wallpaper folder",
                description: "Open ~/Pictures/Wallpapers.",
                button: "Open",
                keywords: "folder wallpapers backgrounds pictures",
                action: SettingsAction::Open("Pictures/Wallpapers"),
            },
        ],
    },
];

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.settings")
        .build();

    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Aurora Settings")
        .default_width(980)
        .default_height(560)
        .decorated(false)
        .resizable(true)
        .build();

    window.add_css_class("settings-window");
    window.set_opacity(0.96);

    let selected_category = Rc::new(Cell::new(None::<usize>));
    let search_entry = SearchEntry::builder()
        .placeholder_text("Search settings, actions, or keywords")
        .hexpand(true)
        .build();
    search_entry.add_css_class("settings-search");

    let content = GtkBox::new(Orientation::Vertical, 16);
    content.add_css_class("settings-content");

    let navigation = build_navigation(&content, &search_entry, &selected_category);
    let sidebar = build_sidebar(&navigation, &window);

    let title = Label::builder()
        .label("Settings")
        .halign(Align::Start)
        .build();
    title.add_css_class("settings-title");

    let subtitle = Label::builder()
        .label("Make Aurora feel like yours")
        .halign(Align::Start)
        .build();
    subtitle.add_css_class("settings-subtitle");

    let heading_copy = GtkBox::new(Orientation::Vertical, 3);
    heading_copy.append(&title);
    heading_copy.append(&subtitle);

    let theme_pill = Label::builder()
        .label(&format!("●  {}", current_theme()))
        .halign(Align::Center)
        .build();
    theme_pill.add_css_class("settings-theme-pill");

    let close_button = Button::with_label("×");
    close_button.add_css_class("settings-close");
    close_button.set_tooltip_text(Some("Close (Escape)"));
    close_button.connect_clicked({
        let window = window.clone();
        move |_| window.close()
    });

    let topbar = GtkBox::new(Orientation::Horizontal, 12);
    topbar.add_css_class("settings-topbar");
    topbar.append(&heading_copy);
    topbar.append(&theme_pill);
    topbar.append(&close_button);

    let search_label = Label::new(Some("Find anything"));
    search_label.add_css_class("settings-section-kicker");
    search_label.set_halign(Align::Start);

    let search_hint = Label::new(Some("⌘K / Ctrl+K"));
    search_hint.add_css_class("settings-search-hint");
    let search_row = GtkBox::new(Orientation::Horizontal, 8);
    search_row.add_css_class("settings-search-row");
    search_row.append(&search_entry);
    search_row.append(&search_hint);

    let header = GtkBox::new(Orientation::Vertical, 10);
    header.add_css_class("settings-header");
    header.append(&topbar);
    header.append(&search_label);
    header.append(&search_row);

    let dashboard = build_dashboard_cards();
    content.append(&dashboard);
    let quick_actions = build_quick_actions();
    content.append(&quick_actions);
    append_settings(&content, "", None);

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .child(&content)
        .build();
    scroll.add_css_class("settings-scroll");

    let main = GtkBox::new(Orientation::Vertical, 0);
    main.add_css_class("settings-main");
    main.append(&header);
    main.append(&scroll);

    let root = GtkBox::new(Orientation::Horizontal, 0);
    root.add_css_class("settings-root");
    root.append(&sidebar);
    root.append(&main);

    search_entry.connect_search_changed({
        let content = content.clone();
        let selected_category = selected_category.clone();
        move |entry| append_settings(&content, &entry.text(), selected_category.get())
    });

    let controller = EventControllerKey::new();
    let win = window.clone();
    let search = search_entry.clone();
    let nav = navigation.clone();
    controller.connect_key_pressed(move |_, key, _, state| {
        if key == gdk::Key::Escape {
            win.close();
            return true.into();
        }
        if key == gdk::Key::K && state.contains(gdk::ModifierType::CONTROL_MASK) {
            search.grab_focus();
            return true.into();
        }
        if state.contains(gdk::ModifierType::ALT_MASK) {
            let index = match key {
                gdk::Key::_1 => Some(0),
                gdk::Key::_2 => Some(1),
                gdk::Key::_3 => Some(2),
                gdk::Key::_4 => Some(3),
                gdk::Key::_5 => Some(4),
                gdk::Key::_6 => Some(5),
                gdk::Key::_7 => Some(6),
                gdk::Key::_8 => Some(7),
                gdk::Key::_9 => Some(8),
                _ => None,
            };
            if let Some(index) = index {
                if let Some(row) = nav.row_at_index(index + 1) {
                    nav.select_row(Some(&row));
                    return true.into();
                }
            }
        }
        false.into()
    });
    window.add_controller(controller);
    window.set_child(Some(&root));
    window.present();
}

fn build_sidebar(navigation: &ListBox, window: &ApplicationWindow) -> GtkBox {
    let sidebar = GtkBox::new(Orientation::Vertical, 14);
    sidebar.add_css_class("settings-sidebar");

    let brand = GtkBox::new(Orientation::Horizontal, 10);
    brand.add_css_class("settings-brand");
    let mark = Label::new(Some("✦"));
    mark.add_css_class("settings-brand-mark");
    let brand_name = Label::new(Some("AURORA"));
    brand_name.add_css_class("settings-brand-name");
    brand.append(&mark);
    brand.append(&brand_name);
    sidebar.append(&brand);

    let nav_label = Label::new(Some("WORKSPACE"));
    nav_label.add_css_class("settings-sidebar-label");
    nav_label.set_halign(Align::Start);
    sidebar.append(&nav_label);
    let navigation_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .child(navigation)
        .build();
    navigation_scroll.add_css_class("settings-navigation-scroll");
    sidebar.append(&navigation_scroll);

    let footer = GtkBox::new(Orientation::Vertical, 4);
    footer.add_css_class("settings-sidebar-footer");
    let status = Label::new(Some(&session_summary()));
    status.add_css_class("settings-sidebar-status");
    status.set_wrap(true);
    status.set_xalign(0.0);
    let help = Label::new(Some("Ctrl+K search  ·  Alt+1…9 navigate"));
    help.add_css_class("settings-sidebar-help");
    help.set_wrap(true);
    help.set_xalign(0.0);
    footer.append(&status);
    footer.append(&help);
    sidebar.append(&footer);

    let window = window.clone();
    navigation.connect_row_activated(move |_, row| {
        if row.index() == 0 {
            window.set_title(Some("Aurora Settings"));
        } else if let Some(category) = SETTINGS_CATEGORIES.get((row.index() - 1) as usize) {
            window.set_title(Some(&format!("{} · Aurora Settings", category.title)));
        }
    });
    sidebar
}

fn build_navigation(
    content: &GtkBox,
    search_entry: &SearchEntry,
    selected_category: &Rc<Cell<Option<usize>>>,
) -> ListBox {
    let navigation = ListBox::new();
    navigation.set_selection_mode(SelectionMode::Single);
    navigation.add_css_class("settings-navigation");

    let all = navigation_row("⌂", "Overview", "Everything in Aurora");
    navigation.append(&all);

    for category in SETTINGS_CATEGORIES {
        navigation.append(&navigation_row(
            category_icon(category.title),
            category.title,
            &format!("{} options", category.items.len()),
        ));
    }
    navigation.select_row(navigation.row_at_index(0).as_ref());

    navigation.connect_row_selected({
        let content = content.clone();
        let search_entry = search_entry.clone();
        let selected_category = selected_category.clone();
        move |_, row| {
            let index = row.map(|row| row.index());
            let selected = index.and_then(|index| index.checked_sub(1).map(|value| value as usize));
            selected_category.set(selected);
            append_settings(&content, &search_entry.text(), selected);
        }
    });
    navigation
}

fn navigation_row(icon: &str, title: &str, count: &str) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_activatable(true);
    let content = GtkBox::new(Orientation::Horizontal, 10);
    content.add_css_class("settings-nav-row");

    let glyph = Label::new(Some(icon));
    glyph.add_css_class("settings-nav-icon");
    let copy = GtkBox::new(Orientation::Vertical, 2);
    copy.set_hexpand(true);
    let title_label = Label::new(Some(title));
    title_label.set_xalign(0.0);
    title_label.add_css_class("settings-nav-title");
    let count_label = Label::new(Some(count));
    count_label.set_xalign(0.0);
    count_label.add_css_class("settings-nav-count");
    copy.append(&title_label);
    copy.append(&count_label);
    content.append(&glyph);
    content.append(&copy);
    row.set_child(Some(&content));
    row
}

fn build_quick_actions() -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 8);
    section.add_css_class("settings-quick-section");
    let title = Label::new(Some("Quick controls"));
    title.add_css_class("settings-section-title");
    title.set_halign(Align::Start);
    section.append(&title);

    for group in [
        [
            (
                "↻  Refresh",
                "Waybar + Hyprland",
                SettingsAction::Launch("refresh_system"),
            ),
            (
                "⌘  Palette",
                "Keyboard launcher",
                SettingsAction::Launch("command_palette"),
            ),
            (
                "✦  Theme",
                "Appearance switcher",
                SettingsAction::Launch("theme_switcher"),
            ),
        ],
        [
            (
                "🔊  Volume +",
                "Raise default output",
                SettingsAction::Run(
                    "wpctl",
                    &["set-volume", "-l", "1", "@DEFAULT_AUDIO_SINK@", "5%+"],
                ),
            ),
            (
                "☼  Brightness +",
                "Raise display backlight",
                SettingsAction::Run("brightnessctl", &["-e4", "-n2", "set", "5%+"]),
            ),
            (
                "  Mute",
                "Toggle default output",
                SettingsAction::Run("wpctl", &["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"]),
            ),
        ],
    ] {
        let actions = GtkBox::new(Orientation::Horizontal, 8);
        actions.add_css_class("settings-quick-actions");
        for (label, hint, action) in group {
            let button = Button::with_label(label);
            button.set_hexpand(true);
            button.set_tooltip_text(Some(hint));
            button.add_css_class("settings-quick-button");
            button.connect_clicked(move |_| run_action(&action));
            actions.append(&button);
        }
        section.append(&actions);
    }
    section
}

fn build_dashboard_cards() -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 8);
    section.add_css_class("settings-dashboard");

    let heading = GtkBox::new(Orientation::Horizontal, 8);
    let title = Label::new(Some("Aurora pulse"));
    title.add_css_class("settings-section-title");
    title.set_halign(Align::Start);
    heading.append(&title);
    let hint = Label::new(Some("live session snapshot"));
    hint.add_css_class("settings-dashboard-hint");
    hint.set_halign(Align::End);
    hint.set_hexpand(true);
    heading.append(&hint);
    section.append(&heading);

    let cards = [
        ("THEME", current_theme(), "active profile"),
        (
            "MONITORS",
            hypr_count("monitors", "Monitor "),
            "connected outputs",
        ),
        (
            "WORKSPACES",
            hypr_count("workspaces", "workspace ID"),
            "known workspaces",
        ),
        ("PROCESSES", process_count(), "running processes"),
        ("AUDIO", audio_status(), "default output"),
        ("BRIGHTNESS", brightness_status(), "backlight level"),
    ];

    for group in cards.chunks(3) {
        let row = GtkBox::new(Orientation::Horizontal, 8);
        row.add_css_class("settings-dashboard-row");
        for (label, value, detail) in group {
            row.append(&status_card(label, value, detail));
        }
        section.append(&row);
    }

    section
}

fn status_card(label: &str, value: &str, detail: &str) -> GtkBox {
    let card = GtkBox::new(Orientation::Vertical, 2);
    card.add_css_class("settings-status-card");
    card.set_hexpand(true);

    let kicker = Label::new(Some(label));
    kicker.add_css_class("settings-card-kicker");
    kicker.set_halign(Align::Start);

    let value_label = Label::new(Some(value));
    value_label.add_css_class("settings-card-value");
    value_label.set_halign(Align::Start);
    value_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    let detail_label = Label::new(Some(detail));
    detail_label.add_css_class("settings-card-detail");
    detail_label.set_halign(Align::Start);

    card.append(&kicker);
    card.append(&value_label);
    card.append(&detail_label);
    card
}

fn append_settings(parent: &GtkBox, query: &str, selected_category: Option<usize>) {
    let mut child = parent.first_child();
    while let Some(current) = child {
        child = current.next_sibling();
        if !current.has_css_class("settings-quick-section") {
            parent.remove(&current);
        }
    }

    let query = query.trim().to_lowercase();
    let mut has_results = false;

    for (category_index, category) in SETTINGS_CATEGORIES.iter().enumerate() {
        if query.is_empty()
            && selected_category.is_some()
            && selected_category != Some(category_index)
        {
            continue;
        }
        let category_matches = category.title.to_lowercase().contains(&query)
            || category.description.to_lowercase().contains(&query);

        let matching_items: Vec<_> = category
            .items
            .iter()
            .filter(|item| {
                query.is_empty()
                    || category_matches
                    || item.title.to_lowercase().contains(&query)
                    || item.description.to_lowercase().contains(&query)
                    || item.button.to_lowercase().contains(&query)
                    || item.keywords.to_lowercase().contains(&query)
            })
            .collect();

        if matching_items.is_empty() {
            continue;
        }

        has_results = true;
        add_category(parent, category, &matching_items, query.is_empty());
    }

    if !has_results {
        let empty = Label::builder()
            .label("No settings matched your search.")
            .halign(Align::Center)
            .valign(Align::Center)
            .vexpand(true)
            .build();
        empty.add_css_class("settings-empty");
        parent.append(&empty);
    }
}

fn add_category(
    parent: &GtkBox,
    category: &SettingsCategory,
    items: &[&SettingsItem],
    show_description: bool,
) {
    let section = GtkBox::new(Orientation::Vertical, 10);
    section.add_css_class("settings-category");

    let heading = GtkBox::new(Orientation::Horizontal, 10);
    let icon = Label::new(Some(category_icon(category.title)));
    icon.add_css_class("settings-category-icon");
    let heading_copy = GtkBox::new(Orientation::Vertical, 2);
    heading_copy.set_hexpand(true);
    let title = Label::new(Some(category.title));
    title.set_halign(Align::Start);
    title.add_css_class("settings-category-title");
    heading_copy.append(&title);

    if show_description {
        let description = Label::new(Some(category.description));
        description.set_halign(Align::Start);
        description.set_wrap(true);
        description.add_css_class("settings-category-description");
        heading_copy.append(&description);
    }
    let count = Label::new(Some(&format!("{} options", items.len())));
    count.add_css_class("settings-category-count");
    count.set_valign(Align::Center);

    heading.append(&icon);
    heading.append(&heading_copy);
    heading.append(&count);
    section.append(&heading);

    for item in items {
        add_setting(&section, item);
    }

    parent.append(&section);
}

fn add_setting(parent: &GtkBox, item: &SettingsItem) {
    let row = GtkBox::new(Orientation::Horizontal, 12);
    row.add_css_class("setting-row");

    let copy = GtkBox::new(Orientation::Vertical, 3);
    copy.set_hexpand(true);

    let label = Label::new(Some(item.title));
    label.set_xalign(0.0);
    label.set_wrap(true);
    label.add_css_class("setting-label");

    let description = Label::new(Some(item.description));
    description.set_xalign(0.0);
    description.set_wrap(true);
    description.add_css_class("setting-description");

    copy.append(&label);
    copy.append(&description);

    let button = Button::with_label(item.button);
    button.add_css_class("setting-button");
    button.set_valign(Align::Center);
    button.set_tooltip_text(Some(item.description));

    button.connect_clicked({
        let action = item.action;
        move |_| run_action(&action)
    });

    row.append(&copy);
    row.append(&button);

    parent.append(&row);
}

fn category_icon(title: &str) -> &'static str {
    match title {
        "Appearance" => "◈",
        "Hyprland" => "⌘",
        "Shell and notifications" => "◌",
        "Aurora tools" => "✦",
        "Session and hardware" => "◉",
        "Configuration files" => "▤",
        "Folders" => "⌂",
        _ => "•",
    }
}

fn run_action(action: &SettingsAction) {
    let home = std::env::var("HOME").unwrap_or_default();
    let terminal = application_from_env("TERMINAL", "kitty");
    let editor = application_from_env("EDITOR", "nvim");
    let file_manager = application_from_env("FILE_MANAGER", "xdg-open");

    let result = match action {
        SettingsAction::Edit(path) => Command::new(&terminal)
            .args(["-e", &editor, &format!("{home}/{path}")])
            .spawn(),
        SettingsAction::Launch(command) => Command::new(command).spawn(),
        SettingsAction::Run(command, args) => Command::new(command).args(*args).spawn(),
        SettingsAction::Open(path) => Command::new(file_manager)
            .arg(format!("{home}/{path}"))
            .spawn(),
    };

    if let Err(err) = result {
        eprintln!("Failed to run settings action: {err}");
    }

    fn application_from_env(variable: &str, fallback: &str) -> String {
        std::env::var(variable)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| String::from(fallback))
    }
}

fn command_output(command: &str, args: &[&str]) -> Option<String> {
    Command::new(command)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|output| !output.is_empty())
}

fn hypr_count(command: &str, marker: &str) -> String {
    let count = command_output("hyprctl", &[command])
        .map(|output| output.lines().filter(|line| line.contains(marker)).count())
        .unwrap_or(0);

    if count == 0 {
        String::from("—")
    } else {
        count.to_string()
    }
}

fn process_count() -> String {
    let count = fs::read_dir("/proc")
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .chars()
                .all(|character| character.is_ascii_digit())
        })
        .count();

    if count == 0 {
        String::from("—")
    } else {
        count.to_string()
    }
}

fn audio_status() -> String {
    command_output("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .and_then(|output| {
            let percentage = output
                .split_whitespace()
                .find_map(|part| part.parse::<f32>().ok())
                .map(|volume| format!("{:.0}%", volume * 100.0))?;
            let muted = output.contains("[MUTED]");
            Some(if muted {
                format!("{percentage} muted")
            } else {
                percentage
            })
        })
        .unwrap_or_else(|| String::from("—"))
}

fn brightness_status() -> String {
    command_output("brightnessctl", &["-m"])
        .and_then(|output| {
            output
                .split(',')
                .find(|part| part.trim_end().ends_with('%'))
                .map(|value| value.trim().to_string())
        })
        .unwrap_or_else(|| String::from("—"))
}

fn session_summary() -> String {
    format!(
        "Theme: {}\n{} monitors · {} workspaces",
        current_theme(),
        hypr_count("monitors", "Monitor "),
        hypr_count("workspaces", "workspace ID")
    )
}

fn current_theme() -> String {
    let theme = dirs::home_dir()
        .map(|home| home.join(".local/share/Aurora/theme_name.log"))
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| String::from("unknown"));

    theme
}
