#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./install.sh" 2>&1
  exit 1
fi

username=$(id -u -n 1000)
builddir=$(pwd)

dnf update -y
dnf upgrade -y

#Install basic packages
dnf install gcc python3 btop firefox fzf tldr neofetch tree ca-certificates curl gnupg cowsay trash-cli util-linux-user -y

# Add firefox backup
gpg $builddir/linux-installer/apps-settings/firefox.backup.tar.bz2.gpg
tar -xf $builddir/linux-installer/apps-settings/firefox.backup.bz2
rm -r $builddir/linux-installer/apps-settings/firefox.backup.bz2

#Install and configure nvim
dnf install neovim -y
git clone https://github.com/AstroNvim/AstroNvim ~/.config/nvim
git clone https://github.com/matteo-luraghi/astro-nvimsetup ~/.config/nvim/lua/user

#Move wallpaper
mkdir /home/$username/Pictures
mkdir /home/$username/Pictures/Wallpapers
cp wallpaper.jpg /home/$username/Pictures/
