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

-- ##############################
-- ### HYPRLAND CONFIGURATION ###
-- ##############################

-- See https://wiki.hypr.land/Configuring/Start/ :)

-- get the home folder
local home = os.getenv("HOME")

-- get the configs folder
local configs = home .. "/.config/hypr/configs"

-- load default monitor configuration (you can use nwg-look)
require("monitor")

-- load default keybinds
dofile(configs .. "/keybinds.lua")

--load layout configs
dofile(configs .. "/layout_configs.lua")

-- load decoration file
dofile(configs .. "/look_and_feel.lua")

-- load autostart programs config
dofile(configs .. "/autostart.lua")

-- load input configuration
dofile(configs .. "/input.lua")

-- load default window rules
dofile(configs .. "/window_rules.lua")

-- load default layer rules
dofile(configs .. "/layer_rules.lua")

-- load default animations
dofile(configs .. "/animations.lua")

-- load env variables
dofile(configs .. "/env_vars.lua")

-- Load all user configurations
-- Users can write their Hyprland configurations here. It is going to be ignored when updating in future
local user_configs_path = home .. "/.config/hypr/User/configs"

local handle = io.popen('find "' .. user_configs_path .. '" -type f -name "*.lua"')
for file in handle:lines() do
    dofile(file)
end

handle:close()

-- load theme based configurations 
dofile(home.."/.config/hypr/Theme/theme.lua")
