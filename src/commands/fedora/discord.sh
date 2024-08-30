#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./discord.sh" 2>&1
	exit 1
fi

# Install alacritty
dnf install discord -y

# Make Discord screen sharing work: uncomment WaylandEnable=false
sed -i '/^#WaylandEnable=false/s/^#//' /etc/gdm3/custom.conf
