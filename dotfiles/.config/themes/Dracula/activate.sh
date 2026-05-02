#!/usr/bin/env bash

set -e

pip install --user gnomelooks

echo 1 | gnomelooks get https://www.gnome-look.org/p/1687249/

gsettings set org.gnome.desktop.interface gtk-theme "Dracula"

notify-send "Theme Applied" "Dracula is now active"