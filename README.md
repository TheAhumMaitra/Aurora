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

<div align="center">
<br>
  <a href="#preview"><kbd> <br> Preview <br> </kbd></a>&ensp;&ensp;
  <a href="#installation"><kbd> <br> Installation <br> </kbd></a>&ensp;&ensp;
  <a href="https://aurorawiki.vercel.app"><kbd> <br> Wiki <br> </kbd></a>
</div>
<br>

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
hyprland (You can use Hyprland git version also)
xdg-desktop-portal-hyprland (you can use git version also)
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
yaru-gtk-theme (AUR)
yaru-icon-theme (AUR)
kitty
neovim
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
uv
sudo-rs
nordzy-hyprcursors (AUR)
rofi-emoji
ttf-jetbrains-mono-nerd
mise
starship
wiremix
wifitui-bin (AUR)
weathr-bin (AUR)
bluetui
btop
jolt (AUR)
leenfetch (AUR)
zen-browser-bin (AUR)
hyprshutdown
```

### Step 2. Clone this repository

#### Clone this repo using `git`
`git clone https://github.com/TheAhumMaitra/Aurora.git`

#### Copy all contents of `Aurora/dotfiles/.config`
`cp -r ./Aurora/dotfiles/.config/* ~/.config/ `

### Step 3. Compile and install scripts of Aurora

#### Go to `.config/hypr/scripts`
`cd ~/.config/hypr/scripts`

#### Install them 

Run - `cargo install --path .`

**Note : You need to install compiler of Rust to do that**


### Install Step 4. Install Wallpaper Switcher

#### Clone the custom Waytrogen repo
```bash
git clone https://github.com/TheAhumMaitra/waytrogen-aurora.git
```
#### Go to the repo folder
```bash
cd waytrogen-aurora
```

#### Install it
```bash
cargo install --path .
```
### Copy and compile gsettings schema
```bash
sudo cp ./org.Waytrogen.Waytrogen.gschema.xml \
/usr/share/glib-2.0/schemas/
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
```

### Step 5. Install LazyVim starter for Neovim
```bash
mv ~/.config/nvim{,.bak}
mv ~/.local/share/nvim{,.bak}
mv ~/.local/state/nvim{,.bak}
mv ~/.cache/nvim{,.bak}
git clone https://github.com/LazyVim/starter ~/.config/nvim
rm -rf ~/.config/nvim/.git
```

# License
GNU Public License V3 or later
