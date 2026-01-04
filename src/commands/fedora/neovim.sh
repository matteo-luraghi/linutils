#!/bin/bash

# Install neovim
sudo dnf install neovim -y

# Setup neovim config
mkdir -p /home/$USER/.config/nvim
git clone --depth 1 https://github.com/AstroNvim/template ~/.config/nvim
