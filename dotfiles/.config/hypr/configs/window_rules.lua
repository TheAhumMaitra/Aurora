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
	name = "Wallaper Switcher - Waytroegn for Aurora",
	match = { class = "org.Waytrogen.Waytrogen" },
	float = true,
})
