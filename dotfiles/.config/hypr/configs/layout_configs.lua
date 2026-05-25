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

-- ############################
-- ### LAYOUT CONFIGURATION ###
-- ############################

-- See https://wiki.hypr.land/Configuring/Scrolling-Layout/
hl.config({
	scrolling = {
		fullscreen_on_one_column = true,
		column_width = 0.8,
		focus_fit_method = 1
	},

	-- See https://wiki.hypr.land/Configuring/Dwindle-Layout/
	dwindle = {
		preserve_split = true,
		-- pseudotile is controlled via keybind usually
	},

	-- See https://wiki.hypr.land/Configuring/Master-Layout/
	master = {
		new_status = "master",
	},

	-- -- See https://wiki.hypr.land/Configuring/Variables/#misc
	-- misc = {
	--   force_default_wallpaper = -1,
	--   disable_hyprland_logo = false,
	-- },
})
