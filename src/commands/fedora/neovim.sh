#!/bin/bash

# Install neovim
sudo dnf install neovim -y

# Setup neovim config
mkdir -p /home/$USER/.config/nvim
git clone https://github.com/matteo-luraghi/nvim /home/$USER/.config/nvim/
