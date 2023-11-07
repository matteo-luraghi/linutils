#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./install.sh" 2>&1
  exit 1
fi

username=$(id -u -n 1000)
builddir=$(pwd)

apt update
apt upgrade -y

apt install nala -y

#Prepare firefox installation
snap remove firefox
sudo add-apt-repository ppa:mozillateam/ppa
echo '
Package: *
Pin: release o=LP-PPA-mozillateam
Pin-Priority: 1001
' | sudo tee /etc/apt/preferences.d/mozilla-firefox
#Prevents updates to install snap firefox
echo 'Unattended-Upgrade::Allowed-Origins:: "LP-PPA-mozillateam:${distro_codename}";' | sudo tee /etc/apt/apt.conf.d/51unattended-upgrades-firefox
#Install basic packages
nala install build-essential vim python3 btop ffmpeg firefox fzf tldr neofetch tree ca-certificates curl gnupg cowsay -y
# Add firefox backup
cd $builddir
gpg $builddir/linux-installer/apps-settings/firefox.backup.tar.bz2.gpg
tar -xf $builddir/linux-installer/apps-settings/firefox.backup.bz2
rm -r $builddir/linux-installer/apps-settings/firefox.backup.bz2

#Install and configure nvim
cd $builddir
curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
chmod u+x nvim.appimage
sudo mv nvim.appimage /usr/local/bin/nvim
git clone https://github.com/AstroNvim/AstroNvim ~/.config/nvim
git clone https://github.com/matteo-luraghi/astro-nvimsetup ~/.config/nvim/lua/user

#Install docker
cd $builddir
dockerVersion=docker-desktop-4.25.0-amd64.deb
modprobe kvm
sudo install -m 0755 -d /etc/apt/keyrings -y
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
nala update
nala install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
wget https://desktop.docker.com/linux/main/amd64/$dockerVersion
chmod u+x $dockerVersion
nala install ./$dockerVersion -y
rm $dockerVersion

#Change Wallpaper
cd $builddir
gsettings set set org.gnome.desktop.background picture-uri-dark file:///home/$username/linux-installer/wallpaper.jpg
gsettings set set org.gnome.desktop.background picture-uri file:///home/$username/linux-installer/wallpaper.jpg

#Install and change shell
cd $builddir
./linux-installer/ubuntu/shell.sh
