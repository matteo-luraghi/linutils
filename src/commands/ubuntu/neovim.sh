#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./neovim.sh" 2>&1
  exit 1
fi

builddir=$(pwd)

# Install and configure neovim
cd $builddir
curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
chmod u+x nvim.appimage
mv nvim.appimage /usr/local/bin/nvim
mkdir ~/.config/nvim
git clone https://github.com/matteo-luraghi/nvim ~/.config/nvim/
