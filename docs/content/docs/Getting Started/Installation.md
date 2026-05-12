---
title: "Installation"
weight: 1
---

> [!IMPORTANT]
Aurora is meant to be used for Arch Linux. But it can be used in many distros

There are two ways of installing Aurora :-

**1. Using installation script    (Arch Linux only)**

**2. Manual installation**


You can use our installation script but it is in beta, might not work properly.

**To install Aurora using installation script follow these steps**

### Step 1
Clone this repo into your home directory

```bash
cd ~ && git clone https://github.com/TheAhumMaitra/Aurora.git
```

### Step 2 - Make the installation script executable and run it
```bash
chmod +x install.sh && ./install.sh
```

### Done
It may ask some questions but it will INstall Aurora


## Manual installation

## Prequirements
You need to install rust

## Installation script

#### Step 1. Install all required + recommended packages :- 

##### Recommended Packages
Make sure your package manger has these packages. This packages will give you the best experience of Aurora
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
uv
sudo-rs
nordzy-hyprcursors (AUR)
rofi-emoji
ttf-jetbrains-mono-nerd
mise
starship
```

> [!WARNING]
> You can use `Hyprland-git`. It  might be very unstable 

### Step 2. Clone this repository

#### Clone this repo using `git`
```bash
git clone https://github.com/TheAhumMaitra/Aurora.git
```

#### Copy all contents of `Aurora/dotfiles/.config`
```bash
cp -r ./Aurora/dotfiles/.config/* ~/.config/ 
```

### Step 3. Compile and install scripts of Aurora

#### Go to `.config/hypr/scripts`

```bash
cd ~/.config/hypr/scripts
```

#### Install them

```bash
cargo install --path .
```

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
### Done
Enjoy Aurora!
