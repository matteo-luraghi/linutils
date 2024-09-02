#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./lazygit.sh" 2>&1
	exit 1
fi

# Install lazygit
dnf copr enable atim/lazygit -y
dnf install lazygit -y

# Copy lazygit config
cp ~/linutils/src/utils/lazygit.yml ~/.config/lazygit/config.yml
