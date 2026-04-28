local home = os.getenv("HOME")
local colors_file = dofile(home.."/.config/hypr/colors.lua")

hl.config({
        general = {
            gaps_in = 8,
            gaps_out = 7,
            border_size = 2,

            col = {
                active_border = colors_file.main,
                inactive_border = colors_file.accent
            },

            resize_on_border = false,
            allow_tearing = false,
            layout = "master",
        },

        decoration = {
            rounding = 10,
            rounding_power = 2,

            shadow = {
                enabled = true,
                range = 4,
                render_power = 3,
                color = colors_file.shadow,
            },

            blur = {
                enabled = true,
                size = 11,
                passes = 3,
                ignore_opacity = true,
                new_optimizations = true,
                xray = true,
            },
        },
        animations = {
    enabled = true,
},
    })


