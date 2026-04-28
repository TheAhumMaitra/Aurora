-- See https://wiki.hypr.land/Configuring/Scrolling-Layout/
hl.config({
  scrolling = {
    fullscreen_on_one_column = true,
    column_width = 0.8,
  },

  -- See https://wiki.hypr.land/Configuring/Dwindle-Layout/
  dwindle = {
    preserve_split = true,
    -- pseudotile is controlled via keybind usually
  },

  -- See https://wiki.hypr.land/Configuring/Master-Layout/
  master = {
    new_status = "master",
  },

  -- -- See https://wiki.hypr.land/Configuring/Variables/#misc
  -- misc = {
  --   force_default_wallpaper = -1,
  --   disable_hyprland_logo = false,
  -- },
})