#!/bin/bash

builddir=$(pwd)

# Needed to run appimages
sudo apt install libfuse2 -y

# Install and configure neovim
cd $builddir
curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
chmod u+x nvim.appimage
sudo mv nvim.appimage /usr/local/bin/nvim
rm -rf ~/.config/nvim
git clone https://github.com/matteo-luraghi/nvim ~/.config/nvim/
