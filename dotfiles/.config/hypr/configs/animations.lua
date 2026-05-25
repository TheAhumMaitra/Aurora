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

-- ###################
-- ### ANIMATIONS ####
-- ###################

-- See https://wiki.hypr.land/Configuring/Advanced-and-Cool/Animations/

-- Curves (converted from old bezier)
hl.curve("easeOut", { type = "bezier", points = { { 0.22, 1 }, { 0.36, 1 } } })
hl.curve("easeIn", { type = "bezier", points = { { 0.4, 0 }, { 1, 1 } } })
hl.curve("smooth", { type = "bezier", points = { { 0.25, 0.1 }, { 0.25, 1 } } })

-- Animations
hl.animation({ leaf = "global", enabled = true, speed = 10, bezier = "default" })
hl.animation({ leaf = "windows", enabled = true, speed = 4.5, bezier = "easeOut" })
hl.animation({ leaf = "windowsIn", enabled = true, speed = 4, bezier = "easeOut", style = "popin 85%" })
hl.animation({ leaf = "windowsOut", enabled = true, speed = 3.5, bezier = "easeIn", style = "popin 85%" })

hl.animation({ leaf = "fade", enabled = true, speed = 3.5, bezier = "smooth" })
hl.animation({ leaf = "fadeIn", enabled = true, speed = 2.5, bezier = "smooth" })
hl.animation({ leaf = "fadeOut", enabled = true, speed = 2.5, bezier = "smooth" })

hl.animation({ leaf = "border", enabled = true, speed = 8, bezier = "easeOut" })

hl.animation({ leaf = "layers", enabled = true, speed = 3.5, bezier = "easeOut" })
hl.animation({ leaf = "layersIn", enabled = true, speed = 3, bezier = "easeOut", style = "fade" })
hl.animation({ leaf = "layersOut", enabled = true, speed = 2.5, bezier = "easeIn", style = "fade" })

hl.animation({ leaf = "workspaces", enabled = true, speed = 5, bezier = "easeOut", style = "slide" })
