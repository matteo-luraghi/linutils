#!/bin/bash

# Install neovim
sudo dnf install neovim -y

# Setup neovim config
mkdir ~/.config/nvim
git clone https://github.com/matteo-luraghi/nvim ~/.config/nvim/
