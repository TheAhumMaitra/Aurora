-- ###################
-- ### KEYBINDINGS ###
-- ###################

-- See https://wiki.hypr.land/Configuring/Keywords/
local home = os.getenv("HOME")
local mainMod = "SUPER" -- Sets "Windows" key as main modifier
local terminal = "kitty"
local fileManager = "nautilus"
local menu = "rofi"
local browser = "google-chrome-stable"
local editor = "code"

-- program binds
hl.bind(mainMod .. " + Q", hl.dsp.exec_cmd(terminal))
hl.bind(mainMod .. " + E", hl.dsp.exec_cmd(fileManager))
hl.bind(mainMod .. " + ALT + B", hl.dsp.exec_cmd(browser))
hl.bind(mainMod .. " + ALT + V", hl.dsp.exec_cmd(editor))
hl.bind(mainMod .. " + R", hl.dsp.exec_cmd(menu .. "-show drun"))
hl.bind(mainMod .. " + ALT + P", hl.dsp.exec_cmd("wlogout"))

-- Aurora's custom gui programs

-- open keybinds help menu
hl.bind(mainMod .. " + H", hl.dsp.exec_cmd(home .. "/.config/hypr/scripts/target/release/keybinds_help"))

-- launch theme switcher
hl.bind(mainMod .. " + T", hl.dsp.exec_cmd(home .. "/.config/hypr/scripts/target/release/theme_switcher"))

-- launch custom settings
hl.bind(mainMod .. " + SHIFT + Z", hl.dsp.exec_cmd(home .. "/.config/hypr/scripts/target/release/settings"))

-- launch search pop up
hl.bind(mainMod .. " + ALT + S", hl.dsp.exec_cmd(home .. "/.config/hypr/scripts/target/release/search"))

-- crucial keybinds

-- Switch workspaces with mainMod + [0-9]
-- Move active window to a workspace with mainMod + SHIFT + [0-9]
for i = 1, 10 do
	local key = i % 10 -- 10 maps to key 0
	hl.bind(mainMod .. " + " .. key, hl.dsp.focus({ workspace = i }))
	hl.bind(mainMod .. " + SHIFT + " .. key, hl.dsp.window.move({ workspace = i }))
end

-- special workspace (scratchpad)
hl.bind(mainMod .. " + S", hl.dsp.workspace.toggle_special("magic"))
hl.bind(mainMod .. " + SHIFT + S", hl.dsp.window.move({ workspace = "special:magic" }))

-- make current window float
hl.bind(mainMod .. " + V", hl.dsp.window.float({ action = "toggle" }))

-- toggle pseudo mode
hl.bind(mainMod .. " + P", hl.dsp.window.pseudo())

-- resize the window
hl.bind(mainMod .. " + SHIFT + RIGHT", hl.dsp.window.resize({ x = 20, y = 0, relative = true }))
hl.bind(mainMod .. " + SHIFT + LEFT", hl.dsp.window.resize({ x = -20, y = 0, relative = true }))
hl.bind(mainMod .. " + SHIFT + UP", hl.dsp.window.resize({ x = 0, y = -20, relative = true }))
hl.bind(mainMod .. " + SHIFT + DOWN", hl.dsp.window.resize({ x = 0, y = 20, relative = true }))

-- Move/resize windows with mainMod + LMB/RMB and dragging
hl.bind(mainMod .. " + mouse:272", hl.dsp.window.drag(), { mouse = true })
hl.bind(mainMod .. " + mouse:273", hl.dsp.window.resize(), { mouse = true })

-- Move focus with mainMod + arrow keys
hl.bind(mainMod .. " + left", hl.dsp.focus({ direction = "left" }))
hl.bind(mainMod .. " + right", hl.dsp.focus({ direction = "right" }))
hl.bind(mainMod .. " + up", hl.dsp.focus({ direction = "up" }))
hl.bind(mainMod .. " + down", hl.dsp.focus({ direction = "down" }))

-- close current window
local closeWindowBind = hl.bind(mainMod .. " + C", hl.dsp.window.close())
closeWindowBind:set_enabled(true)

-- exit Hyprland
hl.bind(
	mainMod .. " + M",
	hl.dsp.exec_cmd("command -v hyprshutdown >/dev/null 2>&1 && hyprshutdown || hyprctl dispatch 'hl.dsp.exit()'")
)

-- rofi menus

-- launch rofi based  emoji menu
hl.bind(mainMod .. " + ALT + E", hl.dsp.exec_cmd(menu .. " -modi emoji -show emoji"))

--launch rofi based clipboard manager
hl.bind(mainMod .. " + SHIFT + V", hl.dsp.exec_cmd("cliphist list | rofi -dmenu | cliphist decode | wl-copy"))

-- to take a screenshot (hyprshot launch)
hl.bind(mainMod .. " + ALT + Z", hl.dsp.exec_cmd("hyprshot -m output"))

-- Scroll through existing workspaces with mainMod + scroll
hl.bind(mainMod .. " + mouse_down", hl.dsp.focus({ workspace = "e+1" }))
hl.bind(mainMod .. " + mouse_up", hl.dsp.focus({ workspace = "e-1" }))

-- layout switch

-- dwindle switch
hl.bind(mainMod .. " + B", hl.dsp.exec_cmd("hyprctl eval 'hl.config({ general = { layout = \"dwindle\" } })'"))

-- master switch
hl.bind(mainMod .. " + K", hl.dsp.exec_cmd("hyprctl eval 'hl.config({ general = { layout = \"master\" } })'"))

-- scrolling switch
hl.bind(mainMod .. " + X", hl.dsp.exec_cmd("hyprctl eval 'hl.config({ general = { layout = \"scrolling\" } })'"))

-- monocle switch
hl.bind(mainMod .. " + Z", hl.dsp.exec_cmd("hyprctl eval 'hl.config({ general = { layout = \"monocle\" } })'"))

-- Laptop multimedia keys for volume and LCD brightness

-- volume keys
hl.bind("XF86AudioRaiseVolume", hl.dsp.exec_cmd("wpctl set-volume -l 1 @DEFAULT_AUDIO_SINK@ 5%+")) --raise volume
hl.bind("XF86AudioLowerVolume", hl.dsp.exec_cmd("wpctl set-volume -l 1 @DEFAULT_AUDIO_SINK@ 5%-")) --decrease volume

-- brightness keys
hl.bind("XF86MonBrightnessUp", hl.dsp.exec_cmd("brightnessctl -e4 -n2 set 5%+")) -- increase brightness
hl.bind("XF86MonBrightnessDown", hl.dsp.exec_cmd("brightnessctl -e4 -n2 set 5%-")) --decrease brightness

-- layout specific keybinds

-- scrolling layout
hl.bind(mainMod .. " + comma", hl.dsp.layout("move -col"))
hl.bind(mainMod .. " + period", hl.dsp.layout("move -col"))
hl.bind(mainMod .. " + SHIFT + comma", hl.dsp.layout("swapcol l"))
hl.bind(mainMod .. " + SHIFT + period", hl.dsp.layout("swapcol r"))
hl.bind(mainMod .. " + ALT + comma", hl.dsp.layout("colresize +0.4"))
hl.bind(mainMod .. " + ALT + period", hl.dsp.layout("colresize +0.4"))

-- monocole layout
hl.bind(mainMod .. " + comma", hl.dsp.layout("cyclenext"))
hl.bind(mainMod .. " + period", hl.dsp.layout("cycleprev"))

-- dwindle layout
hl.bind(mainMod .. " + J", hl.dsp.layout("togglesplit"))
