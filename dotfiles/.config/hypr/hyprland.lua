-- This is an example Hyprland Lua config file.
-- Refer to the wiki for more information.
-- https://wiki.hypr.land/Configuring/Start/

-- Please note not all available settings / options are set here.
-- For a full list, see the wiki

-- You can (and should!!) split this configuration into multiple files
-- Create your files separately and then require them like this:
-- require("myColors")

-- get the home folder
local home = os.getenv("HOME")

-- get the configs folder
local configs = home .. "/.config/hypr/configs"

-- load default monitor configuration (you can use nwg-look)
require("monitor")

-- load default keybinds
dofile(configs .. "/keybinds.lua")

--load layout configs
dofile(configs.."/layout_configs.lua")

-- load decoration file
dofile(configs.."/look_and_feel.lua")

dofile(configs.."/autostart.lua")

dofile(configs.."/input.lua")

dofile(configs.."/window_rules.lua")

dofile(configs.."/layer_rules.lua")

dofile(configs.."/animations.lua")

dofile(configs.."/env_vars.lua")
