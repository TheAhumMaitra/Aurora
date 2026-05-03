---
title: "Creating Themess"
weight: 2
---

Aurora provides developers unique features to customize their theme. In your theme folder, create essential services folders - `hypr`, `rofi`, `waybar`, `wlogout` and a default wallpaper (which should be named `default.png`). You can look our official 4 themes folders for understanding theming structure.

### Example structure of a theme 

```yml
Aurora Default 
      hypr 
         colors.lua
      rofi 
         colors.rasi
      waybar
         colors.css
      wlogout
         colors.css
      default.png #Wallpaper
      config.toml
```
> [!TIP]
> You can create a `config.toml` in your theme folder and you can create a custom script with custom interpreter there

### Example of `config.toml`

```toml
[settings]
script = "default.sh" #we can use default.py or main.py or main.lua
interpreter = "bash" #we can use python, lua, node
```

> [!TIP]
> You can import multiple external scripts on the main script