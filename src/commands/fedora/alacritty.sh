#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./alacritty.sh" 2>&1
	exit 1
fi

# Install alacritty
dnf install alacritty -y

# Setup alacritty
mkdir ~/.config/alacritty
cp ~/linutils/src/utils/alacritty.toml ~/.config/alacritty
