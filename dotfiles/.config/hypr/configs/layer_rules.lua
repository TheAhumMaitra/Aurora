-- layerrule = blur on, match:namespace swaync-control-center
-- layerrule = blur on, match:namespace swaync-notification-window
-- layerrule = ignore_alpha 0, match:namespace swaync-control-center
-- layerrule = ignore_alpha 0, match:namespace swaync-notification-window
-- layerrule = blur on, match:namespace com.aurora.keybinds_help
-- layerrule = ignore_alpha 0, match:namespace com.aurora.keybinds_help
-- layerrule = blur on, match:namespace logout_dialog
-- layerrule = ignore_alpha 0, match:namespace logout_dialog
-- layerrule = blur on, match:namespace com.aurora.search
-- layerrule = ignore_alpha 0, match:namespace com.aurora.search

hl.layer_rule({
	match = { namespace = "swaync-control-center" },
	blur = true,
	ignore_alpha = 0,
})
hl.layer_rule({
	match = { namespace = "swaync-notification-window" },
	blur = true,
	ignore_alpha = 0,
})
hl.layer_rule({
	match = { namespace = "com.aurora.keybinds_help" },
	blur = true,
	ignore_alpha = 0,
})
hl.layer_rule({
	match = { namespace = "logout_dialog" },
	blur = true,
	ignore_alpha = 0,
})
hl.layer_rule({
	match = { namespace = "com.aurora.search" },
	blur = true,
	ignore_alpha = 0,
})
