#!/usr/bin/env bash

set -euo pipefail

THEME_NAME="Dracula"
THEME_URL="https://www.gnome-look.org/p/1687249/"

# Check required commands do exist or not
for cmd in curl gsettings hyprctl; do
    command -v "$cmd" >/dev/null 2>&1 || {
        echo "$cmd is not installed. Install them first!"
        exit 1
    }
done

# Install uv if missing
if ! command -v uv >/dev/null 2>&1; then
    echo "Installing uv..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
fi

# Add uv binaries to PATH
export PATH="$HOME/.local/bin:$PATH"

# Install gnomelooks if missing
if ! command -v gnomelooks >/dev/null 2>&1; then
    echo "Installing gnomelooks..."
    uv tool install gnomelooks
fi

# Create Hyprland theme config directory
mkdir -p "$HOME/.config/hypr/Theme"

# Write theme config for Hyprland
echo "hl.env(\"GTK_THEME\", \"$THEME_NAME\")" \
> "$HOME/.config/hypr/Theme/theme.lua"

# Check if the theme exists 
if [ -d "$HOME/.themes/$THEME_NAME" ] || \
   [ -d "$HOME/.local/share/themes/$THEME_NAME" ]; then
    echo "$THEME_NAME is already installed."
else
    echo "Downloading $THEME_NAME theme..."
    echo 0 | gnomelooks get "$THEME_URL"
fi

# Apply GTK theme
gsettings set org.gnome.desktop.interface gtk-theme "$THEME_NAME"
gsettings set org.gnome.desktop.wm.preferences theme "$THEME_NAME"

# Reload Hyprland
hyprctl reload

echo "Dracula theme installed."

# Notification
command -v notify-send >/dev/null 2>&1 && \
notify-send "Welcome to Dracula" "Enjoy your journey"