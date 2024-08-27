#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./discord.sh" 2>&1
	exit 1
fi

# Install Discord
wget -O discord.deb "https://discord.com/api/download/stable?platform=linux&format=deb"
nala install ./discord.deb -y
rm -r discord.deb

# Make Discord screen sharing work: uncomment WaylandEnable=false
sed -i '/^#WaylandEnable=false/s/^#//' /etc/gdm/custom.conf
