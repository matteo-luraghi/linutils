#!/bin/bash

# Install alacritty
sudo dnf install discord -y

# Make Discord screen sharing work: uncomment WaylandEnable=false
sudo sed -i '/^#WaylandEnable=false/s/^#//' /etc/gdm/custom.conf
