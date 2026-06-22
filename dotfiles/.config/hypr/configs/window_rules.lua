-- SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
-- SPDX-License-Identifier: GPL-3.0-or-later

--   Copyright (C) 2026 Ahum Maitra

--     This program is free software: you can redistribute it and/or modify
--     it under the terms of the GNU General Public License as published by
--     the Free Software Foundation, either version 3 of the License, or
--     (at your option) any later version.

--     This program is distributed in the hope that it will be useful,
--     but WITHOUT ANY WARRANTY; without even the implied warranty of
--     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
--     GNU General Public License for more details.

--     You should have received a copy of the GNU General Public License
--     along with this program.  If not, see <https://www.gnu.org/licenses/>.

-- ####################
-- ### WINDOW RULES ###
-- ####################
local suppressMaximizeRule = hl.window_rule({
	-- Ignore maximize requests from all apps. You'll probably like this.
	name = "suppress-maximize-events",
	match = { class = ".*" },

	suppress_event = "maximize",
})
suppressMaximizeRule:set_enabled(false)

hl.window_rule({
	-- Fix some dragging issues with XWayland
	name = "fix-xwayland-drags",
	match = {
		class = "^$",
		title = "^$",
		xwayland = true,
		float = true,
		fullscreen = false,
		pin = false,
	},

	no_focus = true,
})
-- Hyprland-run windowrule
hl.window_rule({
	name = "move-hyprland-run",
	match = { class = "hyprland-run" },

	move = "20 monitor_h-120",
	float = true,
})
hl.window_rule({
	name = "settings-app",
	match = { class = "com.aurora.settings" },
	float = true,
})
hl.window_rule({
	name = "layout_switcher",
	match = { class = "com.aurora.layout_switcher" },
	float = true,
})
hl.window_rule({
	name = "welcome-app",
	match = { class = "com.aurora.welcome" },
	float = true,
})
hl.window_rule({
	name = "theme-suppressMaximizeRuleswitcher",
	match = { class = "com.aurora.theme_switcher" },
	float = true,
})
hl.window_rule({
	name = "search",
	match = { class = "com.aurora.search" },
	float = true,
})
hl.window_rule({
	name = "keybinds-help",
	match = { class = "com.aurora.keybinds_help" },
	float = true,
})
hl.window_rule({
	name = "Blueman Manager",
	match = { class = "blueman-manager" },
	float = true,
})
hl.window_rule({
	name = "Waybar Position Switcher used by Aurora",
	match = { class = "com.aurora.waybar_position_switcher" },
	float = true,
})
hl.window_rule({
	name = "Wallaper Switcher - Waytroegn for Aurora",
	match = { class = "org.Waytrogen.Waytrogen" },
	float = true,
})
hl.window_rule({
	name = "Weather tui used by Aurora",
	match = { class = "weathr" },
	float = true,
	size = { "monitor_w * 0.7", "monitor_h * 0.7" },
})
hl.window_rule({
	name = "Bluetooth manager used by Aurora",
	match = { class = "bluetui" },
	float = true,
	size = { "monitor_w * 0.7", "monitor_h * 0.7" },
})
hl.window_rule({
	name = "Playback manager used by Aurora",
	match = { class = "wiremix" },
	float = true,
	size = { "monitor_w * 0.7", "monitor_h * 0.7" },
})
hl.window_rule({
	name = "Wifi manager used by Aurora",
	match = { class = "wifitui" },
	float = true,
	size = { "monitor_w * 0.7", "monitor_h * 0.7" },
})
hl.window_rule({
	name = "Graphical system resources monitor used by Aurora",
	match = { class = "btop" },
	float = true,
	size = { "monitor_w * 0.8", "monitor_h * 0.8" },
})
hl.window_rule({
	name = "Battery monitor used by Aurora",
	match = { class = "jolt" },
	float = true,
	size = { "monitor_w * 0.7", "monitor_h * 0.7" },
})
hl.window_rule({
	name = "",
	match = { class = "xdg-desktop-portal-gtk" },
	float = true,
	size = { "monitor_w * 0.7", "monitor_h * 0.7" },
})
-- screensaver
hl.window_rule({
	name = "Aurora screensaver",
	match = { class = "org.aurora.screensaver" },
	fullscreen = true,
})

-- app entries manager
hl.window_rule({
	name = "Aurora App Entires Manager",
	match = { class = "com.aurora.app_entries_home" },
	float = true,
})

-- web app entries center
hl.window_rule({
	name = "Aurora Web App Entires Center",
	match = { class = "com.aurora.web_app_entries_center" },
	float = true,
})

-- web app entry creator
hl.window_rule({
	name = "Aurora Web App Entires Creator",
	match = { class = "com.aurora.web_app_entry_creator" },
	float = true,
})

-- web app entries center
hl.window_rule({
	name = "Aurora TUI App Entires Center",
	match = { class = "com.aurora.tui_app_entries_center" },
	float = true,
})

-- web app entry creator
hl.window_rule({
	name = "Aurora TUI App Entires Creator",
	match = { class = "com.aurora.tui_app_entry_creator" },
	float = true,
})
