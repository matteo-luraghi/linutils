#!/bin/bash

# Allow non-free RPM fusion repo
sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm -y

# Update system
sudo dnf update -y

# Restore minimize and maximize buttons
gsettings set org.gnome.desktop.wm.preferences button-layout 'appmenu:minimize,maximize,close'

# Get permissions to use the brightness control extension
gpasswd --add $USER i2c

# Change wallpaper
gsettings set org.gnome.desktop.background picture-uri-dark file:///home/$username/linutils/src/utils/wallpaper.jpg
gsettings set org.gnome.desktop.background picture-uri file:///home/$username/linutils/src/utils/wallpaper.jpg
