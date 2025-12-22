#!/bin/bash

# Install packages
yes | sudo dnf install hyprland waybar dunst rofi-wayland wlogout xdg-desktop-portal-hyprland qt5-wayland qt6-wayland hyprlock wl-clipboard cliphist mate-polkit

# Copy the configurations
cp -r /home/$USER/linutils/src/utils/dunst /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/hypr /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/ml4w /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/rofi /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/waybar /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/wlogout /home/$USER/.config/
