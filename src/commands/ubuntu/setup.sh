#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./setup.sh" 2>&1
  exit 1
fi

username=$(id -u -n 1000)
builddir=$(pwd)

apt update
apt upgrade -y

echo "deb http://deb.volian.org/volian/ scar main" | sudo tee /etc/apt/sources.list.d/volian-archive-scar-unstable.list; wget -qO - https://deb.volian.org/volian/scar.key | sudo tee /etc/apt/trusted.gpg.d/volian-archive-scar-unstable.gpg

# Remove snap
snap remove --purge firefox
snap remove --purge snap-store
snap remove --purge gnome-3-38-2004
snap remove --purge gnome-42-2204
snap remove --purge gtk-common-themes
snap remove --purge snapd-desktop-integration
snap remove --purge bare
snap remove --purge core20
snap remove --purge canonical-livepatch
snap remove --purge cups
snap remove --purge core22
snap remove core --revision 16091
snap remove --purge snapd
apt remove --autoremove snapd -y
# File to not resintall snap
echo 'Package: snapd
Pin: release a=*
Pin-Priority: -10
' | sudo tee /etc/apt/preferences.d/nosnap.pref
rm -r snap
nala update
apt install --install-suggests gnome-software -y

# Prepare firefox installation
cd $builddir
sudo add-apt-repository ppa:mozillateam/ppa
echo '
Package: *
Pin: release o=LP-PPA-mozillateam
Pin-Priority: 1001
' | sudo tee /etc/apt/preferences.d/mozilla-firefox
# Prevents updates to install snap firefox
echo 'Unattended-Upgrade::Allowed-Origins:: "LP-PPA-mozillateam:${distro_codename}";' | sudo tee /etc/apt/apt.conf.d/51unattended-upgrades-firefox

# Get permissions to use the brightness control extension
gpasswd --add $USER i2c

# Change Wallpaper
cd $builddir
gsettings set org.gnome.desktop.background picture-uri-dark file:///home/$username/linutils/src/utils/wallpaper.jpg
gsettings set org.gnome.desktop.background picture-uri file:///home/$username/linutils/src/utils/wallpaper.jpg
