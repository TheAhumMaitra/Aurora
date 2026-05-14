---
title: "Creating Themess"
weight: 2
---

Aurora provides developers unique features to customize their theme. In your theme folder, create essential services folders - `hypr`, `rofi`, `waybar`, `wlogout` and a default wallpaper (which should be named `default.png`). You can look our official 4 themes folders for understanding theming structure. You can add many wallpapers on the `background` folder

### Example structure of a theme 

```yml
Aurora Default 
      backgrounds
         wallpaper1.png
         wallpaper2.png
         wallpaper3.png
         main.png #If you want users to switch to default background also
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
> You can create a `config.toml` in your theme folder and you can create a custom script with custom interpreter there. A config toml also keeps theme's information

### Example of `config.toml`

```toml
name = "Aurora Default"
version = "0.1.0"

# this is optional
authors = [
    "NAME (@GITHUB_USER_NAME<Optional>) EMAIL",
]

# this is optional 
repo_url = "https://github.com/UserName/ThemeRepo.git"

# this is optional but highly recommended 
wallpaper_sources = ["https://credit.blahblah/blah", "https://blah.nalahbalahbalah"]

# this is optional but highly recommended 
license = "GPL-3.0-or-later"

# this is optional but highly recommended 
[settings]
script = "default.py" #we can use default.py or main.py or main.lua
interpreter = "python" #we can use bash, lua, node
```

> [!TIP]
> You can import multiple external scripts on the main script