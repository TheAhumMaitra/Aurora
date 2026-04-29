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

-- #####################
-- ### LOOK AND FEEL ###
-- #####################

local home = os.getenv("HOME")
local colors_file = dofile(home .. "/.config/hypr/colors.lua")

hl.config({
	general = {
		gaps_in = 8,
		gaps_out = 7,
		border_size = 2,

		col = {
			active_border = colors_file.main,
			inactive_border = colors_file.accent,
		},

		resize_on_border = false,
		allow_tearing = false,
		layout = "master",
	},

	decoration = {
		rounding = 10,
		rounding_power = 2,
		active_opacity = 1.0,
		inactive_opacity = 1.0,

		shadow = {
			enabled = true,
			range = 4,
			render_power = 3,
			color = colors_file.shadow,
		},

		blur = {
			enabled = true,
			size = 11,
			passes = 3,
			ignore_opacity = true,
			new_optimizations = true,
			xray = true,
		},
	},
	animations = {
		enabled = true,
	},
})
