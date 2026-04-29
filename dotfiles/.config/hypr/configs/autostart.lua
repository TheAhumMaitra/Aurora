-- #################
-- ### AUTOSTART ###
-- #################

-- # Autostart necessary processes (like notifications daemons, status bars, etc.)
-- # Or execute your favorite apps at launch like this:

-- # exec-once = $terminal
-- # exec-once = nm-applet &
-- exec-once = hyprsettings -d -H
-- exec-once = waybar
-- exec-once = awww-daemon
-- exec-once = hyprpm reload
-- exec-once = swaync
-- exec-once = hypridle
-- exec-once = $HOME/.config/hypr/scripts/target/release/welcome_app
-- exec-once = systemctl --user start hyprpolkitagent

-- # load cliphist configuration
-- exec-once = wl-paste --type text --watch cliphist store #Stores only text data
-- exec-once = wl-paste --type image --watch cliphist store #Stores only image data

local home = os.getenv("HOME")
hl.on("hyprland.start", function()
	hl.exec_cmd("awww-daemon")
	hl.exec_cmd("waybar & swaync")
	hl.exec_cmd("wl-paste --type text --watch cliphist store ")
	hl.exec_cmd("hypridle")
	hl.exec_cmd(home .. "/.config/hypr/scripts/target/release/welcome_app")
end)
