#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./install.sh" 2>&1
  exit 1
fi

username=$(id -u -n 1000)

dnf update -y
dnf upgrade -y

#Install basic packages
dnf install gcc python3 btop firefox fzf tldr neofetch tree ca-certificates curl gnupg cowsay util-linux-user -y

#Install and configure nvim
dnf install neovim -y
git clone https://github.com/AstroNvim/AstroNvim ~/.config/nvim
git clone https://github.com/matteo-luraghi/astro-nvimsetup ~/.config/nvim/lua/user

#Install docker
dockerVersion=docker-desktop-4.25.0-x86_64.rpm
dnf install dnf-plugins-core -y
dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
dnf install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
wget https://desktop.docker.com/linux/main/amd64/$dockerVersion
chmod u+x $dockerVersion
dnf install ./$dockerVersion -y
rm $dockerVersion

#Move wallpaper
mkdir /home/$username/Pictures
mkdir /home/$username/Pictures/Wallpapers
cp wallpaper.jpg /home/$username/Pictures/

#Update and reboot
dnf update -y
dnf upgrade -y
