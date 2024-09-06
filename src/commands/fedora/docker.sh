#!/bin/bash

# Set up the repository
sudo dnf -y install dnf-plugins-core
sudo dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo

# Install Docker
sudo dnf install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y

# Download docker-desktop
wget -O docker.rpm https://desktop.docker.com/linux/main/amd64/docker-desktop-x86_64.rpm\?utm_source\=docker\&utm_medium\=webreferral\&utm_campaign\=docs-driven-download-linux-amd64

# Install docker-desktop
sudo dnf install docker-desktop.rpm -y

# Remove installation file
rm docker-desktop.rpm
