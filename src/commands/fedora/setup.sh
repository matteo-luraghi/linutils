#!/bin/bash

# Use parallel downloads in dnf
FILE="/etc/dnf/dnf.conf"

# The line to add
LINE="max_parallel_downloads=10"

# Check if the line already exists
if ! grep -q "^$LINE" "$FILE"; then
    # If the line does not exist, append it
    echo "$LINE" | sudo tee -a "$FILE" > /dev/null
    echo "Line added: $LINE"
else
    echo "Line already exists: $LINE"
fi

# Update system
sudo dnf update -y

# Restore minimize and maximize buttons
gsettings set org.gnome.desktop.wm.preferences button-layout 'appmenu:minimize,maximize,close'

# Get permissions to use the brightness control extension
gpasswd --add $USER i2c

# Change wallpaper
cp /home/$USER/linutils/src/utils/wallpaper.jpg /home/$USER/Pictures/
gsettings set org.gnome.desktop.background picture-uri-dark file:///home/$USER/Pictures/wallpaper.jpg
gsettings set org.gnome.desktop.background picture-uri file:///home/$USER/Pictures/wallpaper.jpg

# Switch windows with alt+tab
gsettings set org.gnome.shell.app-switcher current-workspace-only false
gsettings set org.gnome.desktop.wm.keybindings switch-windows "['<Alt>Tab']"

# Setup the dock
gsettings set org.gnome.shell favorite-apps "['org.gnome.Nautilus.desktop', 'firefox.desktop', 'kitty.desktop']"

# Setup battery percentage
git@github.com:matteo-luraghi/linutils.git
