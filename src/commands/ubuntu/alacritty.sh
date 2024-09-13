#!/bin/bash

# Install alacritty
sudo add-apt-repository ppa:aslatter/ppa -y
sudo apt update -y
sudo apt install alacritty -y

# Setup alacritty
mkdir -p ~/.config/alacritty
cp ~/linutils/src/utils/alacritty.toml ~/.config/alacritty

# Open terminal with ctrl+alt+tab
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/']"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ name 'alacritty'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ command 'alacritty'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom0/ binding '<Control><Alt>T'
