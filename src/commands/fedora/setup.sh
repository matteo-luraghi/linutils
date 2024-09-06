#!/bin/bash

# Update system
sudo dnf update -y

# Restore minimize and maximize buttons
gsettings set org.gnome.desktop.wm.preferences button-layout 'appmenu:minimize,maximize,close'

# Get permissions to use the brightness control extension
gpasswd --add $USER i2c

# Change wallpaper
gsettings set org.gnome.desktop.background picture-uri-dark file:///home/$USER/linutils/src/utils/wallpaper.jpg
gsettings set org.gnome.desktop.background picture-uri file:///home/$USER/linutils/src/utils/wallpaper.jpg

# Set the custom keybinding for opening Alacritty
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/']"

# Define the new keybinding command
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name 'Alacritty'

# Set the command to open Alacritty
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command 'alacritty'

# Set the keybinding (Ctrl+Alt+T)
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding '<Control><Alt>T'
