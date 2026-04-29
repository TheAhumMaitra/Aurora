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
