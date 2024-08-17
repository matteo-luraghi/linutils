#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./docker.sh" 2>&1
	exit 1
fi

# Set up the repository
dnf -y install dnf-plugins-core
dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo

# Install Docker
dnf install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y

# Download docker-desktop
wget -O docker.rpm https://desktop.docker.com/linux/main/amd64/docker-desktop-x86_64.rpm\?utm_source\=docker\&utm_medium\=webreferral\&utm_campaign\=docs-driven-download-linux-amd64

# Install docker-desktop
dnf install docker-desktop.rpm

# Remove installation file
rm docker-desktop
