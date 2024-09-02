#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./neovim.sh" 2>&1
	exit 1
fi

# Install neovim
dnf install neovim -y

# Setup neovim config
mkdir ~/.config/nvim
git clone https://github.com/matteo-luraghi/nvim ~/.config/nvim/
