#!/bin/bash

sudo dnf copr enable solopasha/hyprland -y

# Install packages
yes | sudo dnf install hyprland waybar dunst rofi-wayland wlogout xdg-desktop-portal-hyprland qt5-qtwayland qt6-qtwayland hyprlock wl-clipboard cliphist mate-polkit blueman bluez pavucontrol

# Copy the configurations
cp -r /home/$USER/linutils/src/utils/hyprland/dunst /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/hyprland/hypr /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/hyprland/ml4w /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/hyprland/rofi /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/hyprland/waybar /home/$USER/.config/
cp -r /home/$USER/linutils/src/utils/hyprland/wlogout /home/$USER/.config/
