#!/bin/bash

# Install hyprland
sudo dnf install hyprland waybar rofi pipewire wireplumber -y

# Copy configs
cp -r /home/$USER/linutils/src/utils/hypr /home/$USER/.config
cp -r /home/$USER/linutils/src/utils/rofi /home/$USER/.config
cp -r /home/$USER/linutils/src/utils/waybar /home/$USER/.config
