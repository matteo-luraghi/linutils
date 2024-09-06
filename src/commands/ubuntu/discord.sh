#!/bin/bash

# Install Discord
wget -O discord.deb "https://discord.com/api/download/stable?platform=linux&format=deb"
sudo apt install ./discord.deb -y
rm -r discord.deb

# Make Discord screen sharing work: uncomment WaylandEnable=false
# if no argument is passed, otherwise just update discord
if [ -z "$1" ]; then
  sudo sed -i '/^#WaylandEnable=false/s/^#//' /etc/gdm3/custom.conf
fi
