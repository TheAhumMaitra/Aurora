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
