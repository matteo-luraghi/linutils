#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./install.sh" 2>&1
	exit 1
fi

# Allow non-free RPM fusion repo
dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm

# Update system
dnf update -y

# Install basic packages
dnf install vim python3 btop ffmpeg fzf alacritty discord tldr neofetch tree ca-certificates curl gnupg cowsay trash-cli ddcutil -y

# Setup alacritty
mkdir ~/.config/alacritty
cp ~/linux-utils/tools/alacritty.toml ~/.config/alacritty

# Restore minimize and maximize buttons
gsettings set org.gnome.desktop.wm.preferences button-layout 'appmenu:minimize,maximize,close'

# Get permissions to use the brightness control extension
gpasswd --add $USER i2c

# Change wallpaper
gsettings set org.gnome.desktop.background picture-uri-dark file:///home/$username/linux-utils/tools/wallpaper.jpg
gsettings set org.gnome.desktop.background picture-uri file:///home/$username/linux-utils/tools/wallpaper.jpg

# Make Discord screen sharing work: uncomment WaylandEnable=false
nvim /etc/gdm/custom.conf
