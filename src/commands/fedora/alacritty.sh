#!/bin/bash

# Install alacritty
sudo dnf install alacritty -y

# Setup alacritty
mkdir ~/.config/alacritty
cp ~/linutils/src/utils/alacritty.toml ~/.config/alacritty
