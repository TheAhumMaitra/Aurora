-- #################
-- ### AUTOSTART ###
-- #################

hl.on("hyprland.start", function()
	hl.exec_cmd("awww-daemon")
	hl.exec_cmd("waybar & swaync")
	hl.exec_cmd("wl-paste --type text --watch cliphist store ")
	hl.exec_cmd("hypridle")
	hl.exec_cmd("welcome_app")
end)
