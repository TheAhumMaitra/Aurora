#!/usr/bin/env bash

set -e

# Create the directory if it doesn't exist
mkdir -p ~/.config/hypr/Theme

# Write the GTK theme line
echo 'hl.env("GTK_THEME", "Dracula")' > ~/.config/hypr/Theme/theme.lua

# Install gnomelooks
uv tool install gnomelooks --force

# Download the theme
echo 0 | gnomelooks get https://www.gnome-look.org/p/1687249/

# reload the hyprland environment 
hyprctl reload