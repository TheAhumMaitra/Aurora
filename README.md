<!-- SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com> -->
<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

<!-- 
    Copyright (C) 2026 Ahum Maitra

      This program is free software: you can redistribute it and/or modify
      it under the terms of the GNU General Public License as published by
      the Free Software Foundation, either version 3 of the License, or
      (at your option) any later version.

      This program is distributed in the hope that it will be useful,
      but WITHOUT ANY WARRANTY; without even the implied warranty of
      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
      GNU General Public License for more details.

      You should have received a copy of the GNU General Public License
      along with this program.  If not, see <https://www.gnu.org/licenses/>.  -->

# Aurora ™ 

<div align="center">A minimal, elegant, fast Hyprland rice</div>

<br>
<div align="center">

![Aurora name badge](https://img.shields.io/badge/Aurora-cba6f7?style=for-the-badge&labelColor=cba6f7&color=cba6f7)
![Ahum's Project](https://img.shields.io/badge/An_Ahum's_Project-cba6f7?style=for-the-badge&labelColor=cba6f7&color=50fa7b) ![GitHub last commit](https://img.shields.io/github/last-commit/TheAhumMaitra/Aurora?style=for-the-badge&color=b4befe) ![GitHub repo size](https://img.shields.io/github/repo-size/TheAhumMaitra/Aurora?style=for-the-badge&color=cba6f7)
</div>

# Preview
https://github.com/user-attachments/assets/b9e6f020-b497-4b1e-be0a-fd5f6abb959f

# Installation

## Prerequisites

- Rust (install via https://rustup.rs)
- Git
- Arch Linux (recommended)

## Using script
You can use our installation script but it is in beta, might not work properly.

__**To install Aurora using installation script follow these steps**__

### Step 1
Clone this repo into your home directory

`cd ~ && git clone https://github.com/TheAhumMaitra/Aurora.git`

### Step 2 - Make the installation script executable and run it
`chmod +x install.sh && ./install.sh`

## Manually

#### Step 1. Install all required + recommended packages :- 

**For arch**
```
hyprland
wayland
xdg-desktop-portal-hyprland
pipewire
pipewire-pulse
wireplumber
swaync
hypridle
hyprlock
polkit-gnome
waybar
rofi
wlogout
gtk3
gtk4
adwaita-gtk-theme
kitty
cliphist
nautilus
wl-clipboard
hyprshot
network-manager-applet
brightnessctl
libnotify
ttf-dejavu
noto-fonts
noto-fonts-emoji
awww
git
papirus-icon-theme
sudo-rs
```


### Step 2. Clone this repository

#### Clone this repo using `git`
`git clone https://github.com/TheAhumMaitra/Aurora.git`

#### Copy all contents of `Aurora/dotfiles/.config`
`cp -r ./Aurora/dotfiles/.config/* ~/.config/ `

### Step 3. Compile and install scripts of Aurora

#### Go to `.config/hypr/scripts`
`cd ~/.config/hypr/scripts`

#### Install them in 

Run - `cargo install --path .`

**Note : You need to install compiler of Rust to do that**


# License
GNU Public License V3 or later