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

-- #############
-- ### INPUT ###
-- #############

-- # https://wiki.hypr.land/Configuring/Variables/#input
hl.config({
	input = {
		kb_layout = "gb,us",
		kb_variant = "",
		kb_model = "",
		kb_options = "grp:win_space_toggle",
		kb_rules = "",

		follow_mouse = 1,

		sensitivity = 0, -- -1.0 - 1.0, 0 --means no modification.

		touchpad = {
			natural_scroll = true,
		},
	},
})

-- -- # See https://wiki.hypr.land/Configuring/Gestures
-- gesture = 3, horizontal, workspace

-- -- # Example per-device config
-- -- # See https://wiki.hypr.land/Configuring/Keywords/#per-device-input-configs for more
-- device {
--   name = epic-mouse-v1
--   sensitivity = -0.5
-- }
