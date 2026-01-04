#!/bin/bash

# Allow non-free RPM fusion repo
sudo dnf install https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm -y

sudo dnf update -y

# Install discord
sudo dnf install discord -y
