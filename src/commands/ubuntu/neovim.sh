#!/bin/bash

builddir=$(pwd)

# Install and configure neovim
cd $builddir
curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
chmod u+x nvim.appimage
mv nvim.appimage /usr/local/bin/nvim
mkdir ~/.config/nvim
git clone https://github.com/matteo-luraghi/nvim ~/.config/nvim/
